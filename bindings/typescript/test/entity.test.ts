import { describe, it, expect, beforeAll } from 'vitest'
import { createRomcal, Romcal, EntityQuery } from '../src/index.js'
import { loadAllCalendarDefinitions, loadAllResources } from './fixtures.js'

describe('Entity methods', () => {
  let romcal: Romcal

  beforeAll(async () => {
    const calendarDefinitions = await loadAllCalendarDefinitions()
    const resources = await loadAllResources()
    romcal = await createRomcal({
      calendar: 'general_roman',
      locale: 'en',
      calendarDefinitions,
      resources,
    })
  })

  describe('getEntity', () => {
    it('should return an entity by exact ID', () => {
      const entity = romcal.getEntity('agnes_of_rome_virgin')

      expect(entity).not.toBeNull()
      expect(entity!.id).toBe('agnes_of_rome_virgin')
    })

    it('should return null for non-existent entity', () => {
      const entity = romcal.getEntity('non_existent_entity_id')

      expect(entity).toBeNull()
    })

    it('should return entity with expected properties', () => {
      const entity = romcal.getEntity('francis_of_assisi')

      expect(entity).not.toBeNull()
      expect(entity!.id).toBe('francis_of_assisi')
      expect(entity!.name).toBe('Francis of Assisi')
      expect(entity!.canonization_level).toBe('saint')
    })
  })

  describe('searchEntities', () => {
    it('should search entities by text', () => {
      const query: EntityQuery = {
        text: 'francis',
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeGreaterThan(0)
      // Results should be sorted by score (highest first)
      for (let i = 1; i < results.length; i++) {
        expect(results[i - 1].score).toBeGreaterThanOrEqual(results[i].score)
      }
    })

    it('should return EntitySearchResult with correct structure', () => {
      const query: EntityQuery = {
        text: 'agnes',
        limit: 1,
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBe(1)
      const result = results[0]

      // Check EntitySearchResult structure
      expect(result).toHaveProperty('entity')
      expect(result).toHaveProperty('score')
      expect(result).toHaveProperty('match_type')
      expect(result).toHaveProperty('matched_fields')

      // Verify types
      expect(typeof result.score).toBe('number')
      expect(result.score).toBeGreaterThan(0)
      expect(result.score).toBeLessThanOrEqual(1)
      expect(Array.isArray(result.matched_fields)).toBe(true)
    })

    it('should filter by canonization level', () => {
      const query: EntityQuery = {
        canonization_level: 'blessed',
        limit: 10,
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        expect(result.entity.canonization_level).toBe('blessed')
      }
    })

    it('should respect limit parameter', () => {
      const query: EntityQuery = {
        text: 'saint',
        limit: 5,
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeLessThanOrEqual(5)
    })

    it('should return fewer results with high min_score', () => {
      const queryLow: EntityQuery = {
        text: 'john',
        min_score: 0.3,
        limit: 50,
      }
      const queryHigh: EntityQuery = {
        text: 'john',
        min_score: 0.9,
        limit: 50,
      }

      const resultsLow = romcal.searchEntities(queryLow)
      const resultsHigh = romcal.searchEntities(queryHigh)

      // Higher minScore should return fewer or equal results
      expect(resultsHigh.length).toBeLessThanOrEqual(resultsLow.length)
      // All results should be above the minScore threshold
      for (const result of resultsHigh) {
        expect(result.score).toBeGreaterThanOrEqual(0.9)
      }
    })

    it('should return exact ID match with score 1.0', () => {
      const query: EntityQuery = {
        text: 'agnes_of_rome_virgin',
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeGreaterThan(0)
      const exactMatch = results.find((r) => r.entity.id === 'agnes_of_rome_virgin')
      expect(exactMatch).toBeDefined()
      expect(exactMatch!.score).toBe(1.0)
      expect(exactMatch!.match_type).toBe('exact_id')
    })

    it('should filter by titles', () => {
      const query: EntityQuery = {
        titles: ['martyr'],
        limit: 10,
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        expect(result.entity.titles).toBeDefined()
        expect(result.entity.titles).toContain('martyr')
      }
    })

    it('should combine text search with canonization filter', () => {
      const query: EntityQuery = {
        text: 'john',
        canonization_level: 'saint',
        limit: 10,
      }
      const results = romcal.searchEntities(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        // Verify filter is applied
        expect(result.entity.canonization_level).toBe('saint')
        // Verify text search worked (fuzzy match on 'john')
        const hasJohnInFields =
          result.entity.id.toLowerCase().includes('john') ||
          result.entity.name?.toLowerCase().includes('john') ||
          result.entity.fullname?.toLowerCase().includes('john')
        expect(hasJohnInFields).toBe(true)
      }
    })
  })
})
