import { describe, it, expect, beforeAll } from 'vitest'
import { createRomcal, Romcal, MartyrologyQuery } from '../src/index.js'
import { loadAllCalendarDefinitions, loadAllResources } from './fixtures.js'

describe('Martyrology methods', () => {
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

  describe('getMartyrologyEntry', () => {
    it('should return an entry by exact ID', () => {
      const entry = romcal.getMartyrologyEntry('agnes_of_rome_virgin')

      expect(entry).not.toBeNull()
      expect(entry!.id).toBe('agnes_of_rome_virgin')
    })

    it('should return null for non-existent entry', () => {
      const entry = romcal.getMartyrologyEntry('non_existent_entry_id')

      expect(entry).toBeNull()
    })

    it('should return entry with expected properties', () => {
      const entry = romcal.getMartyrologyEntry('francis_of_assisi')

      expect(entry).not.toBeNull()
      expect(entry!.id).toBe('francis_of_assisi')
      expect(entry!.name).toBe('Francis of Assisi')
      expect(entry!.canonization_level).toBe('saint')
    })
  })

  describe('searchMartyrologyEntries', () => {
    it('should search entries by text', () => {
      const query: MartyrologyQuery = {
        text: 'francis',
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeGreaterThan(0)
      // Results should be sorted by score (highest first)
      for (let i = 1; i < results.length; i++) {
        expect(results[i - 1].score).toBeGreaterThanOrEqual(results[i].score)
      }
    })

    it('should return MartyrologySearchResult with correct structure', () => {
      const query: MartyrologyQuery = {
        text: 'agnes',
        limit: 1,
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBe(1)
      const result = results[0]

      // Check MartyrologySearchResult structure
      expect(result).toHaveProperty('entry')
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
      const query: MartyrologyQuery = {
        canonization_level: 'blessed',
        limit: 10,
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        expect(result.entry.canonization_level).toBe('blessed')
      }
    })

    it('should respect limit parameter', () => {
      const query: MartyrologyQuery = {
        text: 'saint',
        limit: 5,
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeLessThanOrEqual(5)
    })

    it('should return fewer results with high min_score', () => {
      const queryLow: MartyrologyQuery = {
        text: 'john',
        min_score: 0.3,
        limit: 50,
      }
      const queryHigh: MartyrologyQuery = {
        text: 'john',
        min_score: 0.9,
        limit: 50,
      }

      const resultsLow = romcal.searchMartyrologyEntries(queryLow)
      const resultsHigh = romcal.searchMartyrologyEntries(queryHigh)

      // Higher minScore should return fewer or equal results
      expect(resultsHigh.length).toBeLessThanOrEqual(resultsLow.length)
      // All results should be above the minScore threshold
      for (const result of resultsHigh) {
        expect(result.score).toBeGreaterThanOrEqual(0.9)
      }
    })

    it('should return exact ID match with score 1.0', () => {
      const query: MartyrologyQuery = {
        text: 'agnes_of_rome_virgin',
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeGreaterThan(0)
      const exactMatch = results.find((r) => r.entry.id === 'agnes_of_rome_virgin')
      expect(exactMatch).toBeDefined()
      expect(exactMatch!.score).toBe(1.0)
      expect(exactMatch!.match_type).toBe('exact_id')
    })

    it('should filter by titles', () => {
      const query: MartyrologyQuery = {
        titles: ['martyr'],
        limit: 10,
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        expect(result.entry.titles).toBeDefined()
        expect(result.entry.titles).toContain('martyr')
      }
    })

    it('should combine text search with canonization filter', () => {
      const query: MartyrologyQuery = {
        text: 'john',
        canonization_level: 'saint',
        limit: 10,
      }
      const results = romcal.searchMartyrologyEntries(query)

      expect(results.length).toBeGreaterThan(0)
      for (const result of results) {
        // Verify filter is applied
        expect(result.entry.canonization_level).toBe('saint')
        // Verify text search worked (fuzzy match on 'john')
        const hasJohnInFields =
          result.entry.id.toLowerCase().includes('john') ||
          result.entry.name?.toLowerCase().includes('john') ||
          result.entry.fullname?.toLowerCase().includes('john')
        expect(hasJohnInFields).toBe(true)
      }
    })
  })
})
