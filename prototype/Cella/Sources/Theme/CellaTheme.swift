import SwiftUI

// MARK: - Color Palette

extension Color {
    // Fond
    static let parchment = Color(red: 0.98, green: 0.969, blue: 0.949)          // #FAF7F2
    static let parchmentWarm = Color(red: 0.941, green: 0.922, blue: 0.89)      // #F0EBE3
    static let inkDark = Color(red: 0.11, green: 0.102, blue: 0.09)             // #1C1A17
    static let inkDarkSecondary = Color(red: 0.165, green: 0.153, blue: 0.133)  // #2A2722

    // Texte
    static let textPrimary = Color(red: 0.173, green: 0.145, blue: 0.125)       // #2C2520
    static let textSecondary = Color(red: 0.478, green: 0.439, blue: 0.404)     // #7A7067
    static let textTertiary = Color(red: 0.659, green: 0.624, blue: 0.584)      // #A89F95

    // Accents
    static let accentLetrine = Color(red: 0.545, green: 0.102, blue: 0.102)     // #8B1A1A
    static let accentGold = Color(red: 0.769, green: 0.584, blue: 0.416)        // #C4956A
    static let separator = Color(red: 0.91, green: 0.886, blue: 0.855)          // #E8E2DA

    // Couleurs liturgiques
    static let liturgicalWhite = Color(red: 0.769, green: 0.584, blue: 0.416)   // or patiné
    static let liturgicalRed = Color(red: 0.545, green: 0.102, blue: 0.102)     // #8B1A1A
    static let liturgicalGreen = Color(red: 0.29, green: 0.404, blue: 0.255)    // #4A6741
    static let liturgicalViolet = Color(red: 0.357, green: 0.227, blue: 0.42)   // #5B3A6B
    static let liturgicalRose = Color(red: 0.69, green: 0.478, blue: 0.541)     // #B07A8A
    static let liturgicalBlack = Color(red: 0.173, green: 0.145, blue: 0.125)   // #2C2520
}

// MARK: - Liturgical Color Enum

enum LiturgicalColor: String, CaseIterable, Identifiable {
    case white, red, green, violet, rose, black, gold

    var id: String { rawValue }

    var color: Color {
        switch self {
        case .white: return .liturgicalWhite
        case .red: return .liturgicalRed
        case .green: return .liturgicalGreen
        case .violet: return .liturgicalViolet
        case .rose: return .liturgicalRose
        case .black: return .liturgicalBlack
        case .gold: return .accentGold
        }
    }

    var label: String {
        switch self {
        case .white: return "Blanc"
        case .red: return "Rouge"
        case .green: return "Vert"
        case .violet: return "Violet"
        case .rose: return "Rose"
        case .black: return "Noir"
        case .gold: return "Doré"
        }
    }
}

// MARK: - Typography

struct CellaFont {
    // Titres liturgiques — Cormorant Garamond
    static func title(_ size: CGFloat) -> Font {
        .custom("Cormorant Garamond", size: size).weight(.semibold)
    }

    static func titleItalic(_ size: CGFloat) -> Font {
        .custom("Cormorant Garamond", size: size).italic()
    }

    // Corps de texte — Georgia (system serif, fallback pour Source Serif)
    static func body(_ size: CGFloat) -> Font {
        .system(size: size, design: .serif)
    }

    // Rubriques, labels — system sans-serif (Inter-like)
    static func label(_ size: CGFloat) -> Font {
        .system(size: size, weight: .medium, design: .default)
    }

    static func labelLight(_ size: CGFloat) -> Font {
        .system(size: size, weight: .regular, design: .default)
    }

    static func caption(_ size: CGFloat) -> Font {
        .system(size: size, weight: .regular, design: .default)
    }
}

// MARK: - Theme Environment

struct CellaTheme {
    let liturgicalColor: LiturgicalColor

    var accentColor: Color { liturgicalColor.color }

    // Spacing
    static let horizontalPadding: CGFloat = 24
    static let verticalSpacing: CGFloat = 16
    static let cardPadding: CGFloat = 20
    static let sectionSpacing: CGFloat = 32

    // Corner radius
    static let cardRadius: CGFloat = 14
    static let smallRadius: CGFloat = 8

    // Line widths
    static let liturgicalBarWidth: CGFloat = 3
}

// MARK: - View Modifiers

struct CardStyle: ViewModifier {
    var liturgicalColor: Color? = nil

    func body(content: Content) -> some View {
        content
            .padding(CellaTheme.cardPadding)
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
            .overlay(alignment: .leading) {
                if let color = liturgicalColor {
                    RoundedRectangle(cornerRadius: 2)
                        .fill(color)
                        .frame(width: CellaTheme.liturgicalBarWidth)
                        .padding(.vertical, 8)
                }
            }
    }
}

struct SectionHeaderStyle: ViewModifier {
    func body(content: Content) -> some View {
        content
            .font(CellaFont.label(11))
            .foregroundStyle(Color.textTertiary)
            .tracking(1.5)
            .textCase(.uppercase)
    }
}

extension View {
    func cellaCard(liturgicalColor: Color? = nil) -> some View {
        modifier(CardStyle(liturgicalColor: liturgicalColor))
    }

    func cellaSectionHeader() -> some View {
        modifier(SectionHeaderStyle())
    }
}
