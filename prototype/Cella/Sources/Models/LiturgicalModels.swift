import Foundation

// MARK: - Liturgical Rank

enum LiturgicalRank: String, Comparable {
    case solemnity = "Solennité"
    case sunday = "Dimanche"
    case feast = "Fête"
    case memorial = "Mémoire"
    case optionalMemorial = "Mémoire facultative"
    case weekday = "Férie"

    private var sortOrder: Int {
        switch self {
        case .solemnity: return 0
        case .sunday: return 1
        case .feast: return 2
        case .memorial: return 3
        case .optionalMemorial: return 4
        case .weekday: return 5
        }
    }

    static func < (lhs: LiturgicalRank, rhs: LiturgicalRank) -> Bool {
        lhs.sortOrder < rhs.sortOrder
    }
}

// MARK: - Season

enum LiturgicalSeason: String {
    case advent = "Avent"
    case christmasTime = "Temps de Noël"
    case lent = "Carême"
    case paschalTriduum = "Triduum pascal"
    case easterTime = "Temps pascal"
    case ordinaryTime = "Temps ordinaire"
}

// MARK: - Celebration

struct Celebration: Identifiable {
    let id: String
    let fullname: String
    let rank: LiturgicalRank
    let color: LiturgicalColor
    let isOptional: Bool
    let martyrologyNote: String?
    let commons: [String]

    var rankLabel: String { rank.rawValue }
}

// MARK: - Liturgical Day

struct LiturgicalDay: Identifiable {
    let id: String
    let date: Date
    let title: String
    let subtitle: String
    let season: LiturgicalSeason
    let color: LiturgicalColor
    let psalterWeek: Int // 1-4
    let sundayCycle: String // A, B, C
    let weekdayCycle: String // I, II
    let celebrations: [Celebration]
    let collect: String
    let prayerOverOfferings: String
    let prayerAfterCommunion: String
    let martyrologyEntry: MartyrologyEntry?
}

// MARK: - Martyrology Entry

struct MartyrologyEntry: Identifiable {
    let id: String
    let fullname: String
    let shortBio: String
    let dateOfDeath: String?
    let titles: [String]
    let canonizationLevel: String?
}

// MARK: - Mass Reading

struct MassReading: Identifiable {
    let id: String
    let type: ReadingType
    let reference: String
    let text: String
    let response: String? // for psalms
    let alternativeIndex: Int? // nil if no alternatives, 0-based if multiple
    let alternativeCount: Int?

    enum ReadingType: String {
        case firstReading = "Première lecture"
        case psalm = "Psaume responsorial"
        case secondReading = "Deuxième lecture"
        case acclamation = "Acclamation de l'Évangile"
        case gospel = "Évangile"
        case sequence = "Séquence"
    }
}

// MARK: - Mass Formulary

struct MassFormulary: Identifiable {
    let id: String
    let name: String // "Messe du jour", "Messe de la vigile", etc.
    let readings: [MassReading]
}

// MARK: - Commentary

struct PatristicCommentary: Identifiable {
    let id: String
    let author: String
    let source: String
    let text: String
    let sourceType: String // "Patristique", "Spirituel", "Magistère"
    let relatedReadingType: MassReading.ReadingType
}

// MARK: - Office (Liturgy of the Hours)

enum OfficeType: String, CaseIterable, Identifiable {
    case officeOfReadings = "Office des lectures"
    case lauds = "Laudes"
    case terce = "Tierce"
    case sext = "Sexte"
    case none_ = "None"
    case vespers = "Vêpres"
    case compline = "Complies"

    var id: String { rawValue }

    var subtitle: String {
        switch self {
        case .officeOfReadings: return "Vigiles"
        case .lauds: return "Prière du matin"
        case .terce: return "Prière du milieu du jour"
        case .sext: return "Prière du milieu du jour"
        case .none_: return "Prière du milieu du jour"
        case .vespers: return "Prière du soir"
        case .compline: return "Prière de la nuit"
        }
    }

    var icon: String {
        switch self {
        case .officeOfReadings: return "moon.stars"
        case .lauds: return "sunrise"
        case .terce: return "sun.and.horizon"
        case .sext: return "sun.max"
        case .none_: return "sun.haze"
        case .vespers: return "sunset"
        case .compline: return "moon"
        }
    }

    var suggestedHourRange: ClosedRange<Int> {
        switch self {
        case .officeOfReadings: return 0...5
        case .lauds: return 6...8
        case .terce: return 9...11
        case .sext: return 11...13
        case .none_: return 14...16
        case .vespers: return 17...20
        case .compline: return 20...23
        }
    }

    var isMidDay: Bool {
        self == .terce || self == .sext || self == .none_
    }

    var components: String {
        switch self {
        case .officeOfReadings: return "Hymne · Psaumes · Lectures"
        case .lauds: return "Hymne · Psaumes · Cantique · Intercessions"
        case .terce, .sext, .none_: return "Hymne · Psaumes · Lecture brève"
        case .vespers: return "Hymne · Psaumes · Cantique · Intercessions"
        case .compline: return "Hymne · Psaume · Cantique de Siméon"
        }
    }
}

// MARK: - Office Content

struct OfficeSection: Identifiable {
    let id = UUID()
    let title: String
    let subtitle: String?
    let content: String
    let isAntiphon: Bool
    let isResponse: Bool

    init(title: String, subtitle: String? = nil, content: String, isAntiphon: Bool = false, isResponse: Bool = false) {
        self.title = title
        self.subtitle = subtitle
        self.content = content
        self.isAntiphon = isAntiphon
        self.isResponse = isResponse
    }
}

struct OfficeContent: Identifiable {
    let id: String
    let type: OfficeType
    let sections: [OfficeSection]
}

// MARK: - Office Source

struct OfficeSource: Identifiable, Hashable {
    let id: String
    let name: String
    let communityName: String?

    static func == (lhs: OfficeSource, rhs: OfficeSource) -> Bool { lhs.id == rhs.id }
    func hash(into hasher: inout Hasher) { hasher.combine(id) }
}

// MARK: - Calendar Settings

struct CalendarRegion: Identifiable {
    let id: String
    let name: String
    let countries: [CalendarCountry]
}

struct CalendarCountry: Identifiable {
    let id: String
    let name: String
    let dioceses: [CalendarDiocese]
}

struct CalendarDiocese: Identifiable {
    let id: String
    let name: String
}

struct ReligiousCommunity: Identifiable {
    let id: String
    let name: String
    let abbreviation: String
    let provinces: [CommunityProvince]
}

struct CommunityProvince: Identifiable {
    let id: String
    let name: String
}
