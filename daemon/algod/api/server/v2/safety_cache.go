package v2

import (
	"container/list"
	"fmt"
	"time"

	"github.com/algorand/go-deadlock"

	"github.com/algorand/go-algorand/crypto"
	"github.com/algorand/go-algorand/protocol"
)

const (
	defaultSafetyCacheCapacity = 1024
	defaultSafetyCacheTTL      = 30 * time.Second
)

type safetyCacheEntry struct {
	key       string
	value     any
	expiresAt time.Time
}

type safetyCache struct {
	mu       deadlock.Mutex
	capacity int
	ttl      time.Duration
	ll       *list.List
	entries  map[string]*list.Element
}

func newSafetyCache(capacity int, ttl time.Duration) *safetyCache {
	if capacity <= 0 {
		capacity = defaultSafetyCacheCapacity
	}
	if ttl <= 0 {
		ttl = defaultSafetyCacheTTL
	}
	return &safetyCache{
		capacity: capacity,
		ttl:      ttl,
		ll:       list.New(),
		entries:  make(map[string]*list.Element, capacity),
	}
}

// NewDefaultSafetyCache returns a bounded, short-lived cache for safety endpoints.
func NewDefaultSafetyCache() *safetyCache {
	return newSafetyCache(defaultSafetyCacheCapacity, defaultSafetyCacheTTL)
}

func (c *safetyCache) get(key string) (any, bool) {
	c.mu.Lock()
	defer c.mu.Unlock()

	elem, ok := c.entries[key]
	if !ok {
		return nil, false
	}
	entry := elem.Value.(*safetyCacheEntry)
	if time.Now().After(entry.expiresAt) {
		c.ll.Remove(elem)
		delete(c.entries, key)
		return nil, false
	}
	c.ll.MoveToFront(elem)
	return entry.value, true
}

func (c *safetyCache) set(key string, value any) {
	c.mu.Lock()
	defer c.mu.Unlock()

	if elem, ok := c.entries[key]; ok {
		entry := elem.Value.(*safetyCacheEntry)
		entry.value = value
		entry.expiresAt = time.Now().Add(c.ttl)
		c.ll.MoveToFront(elem)
		return
	}

	entry := &safetyCacheEntry{
		key:       key,
		value:     value,
		expiresAt: time.Now().Add(c.ttl),
	}
	elem := c.ll.PushFront(entry)
	c.entries[key] = elem

	for c.ll.Len() > c.capacity {
		back := c.ll.Back()
		if back == nil {
			break
		}
		backEntry := back.Value.(*safetyCacheEntry)
		delete(c.entries, backEntry.key)
		c.ll.Remove(back)
	}
}

func safetyCacheKey(prefix string, payload any) string {
	encoded := protocol.EncodeReflect(payload)
	digest := crypto.Hash(encoded)
	return fmt.Sprintf("%s:%x", prefix, digest)
}
