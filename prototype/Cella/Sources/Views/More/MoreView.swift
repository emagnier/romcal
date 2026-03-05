import SwiftUI

// MARK: - More Tab

struct MoreView: View {
    private let liturgicalColor: Color = MockData.today.color.color

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 0) {
                    Rectangle()
                        .fill(liturgicalColor)
                        .frame(height: 3)

                    VStack(spacing: CellaTheme.sectionSpacing) {
                        brandHeader
                        exploreSection
                        settingsSection
                        aboutSection
                    }
                    .padding(.horizontal, CellaTheme.horizontalPadding)
                    .padding(.vertical, CellaTheme.verticalSpacing)
                }
            }
            .background(Color.parchmentWarm)
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    // MARK: - Brand Header

    private var brandHeader: some View {
        VStack(spacing: 8) {
            Text("CELLA")
                .font(.system(size: 28, weight: .light, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .tracking(6)

            Text("« Il m'a introduite dans son cellier… »")
                .font(.system(size: 14, design: .serif).italic())
                .foregroundStyle(Color.textSecondary)

            Text("Ct 2,4")
                .font(CellaFont.caption(12))
                .foregroundStyle(Color.textTertiary)
        }
        .padding(.vertical, 8)
    }

    // MARK: - Explore Section

    private var exploreSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("EXPLORER")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                ExploreRow(icon: "book", title: "Bible", subtitle: "Textes de l'Écriture Sainte", comingSoon: true)
                Divider().padding(.leading, 52)
                ExploreRow(icon: "hands.clap", title: "Recueil de prières", subtitle: "Prières traditionnelles et dévotions", comingSoon: true)
                Divider().padding(.leading, 52)
                ExploreRow(icon: "building.columns", title: "Messe.info", subtitle: "Horaires des messes près de chez vous", comingSoon: true)
                Divider().padding(.leading, 52)
                ExploreRow(icon: "headphones", title: "Podcasts", subtitle: "Audio d'une communauté ou paroisse", comingSoon: true)
                Divider().padding(.leading, 52)
                ExploreRow(icon: "calendar", title: "Agenda paroissial", subtitle: "Événements de votre communauté", comingSoon: true)
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
        }
    }

    // MARK: - Settings Section

    private var settingsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("RÉGLAGES")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                NavigationLink(destination: CalendarSettingsView()) {
                    SettingsRow(icon: "globe", title: "Calendrier & Lieu")
                }
                Divider().padding(.leading, 52)
                NavigationLink(destination: CommunitySettingsView()) {
                    SettingsRow(icon: "person.3", title: "Communauté religieuse")
                }
                Divider().padding(.leading, 52)
                NavigationLink(destination: AppearanceSettingsView()) {
                    SettingsRow(icon: "circle.lefthalf.filled", title: "Apparence")
                }
                Divider().padding(.leading, 52)
                NavigationLink(destination: Text("Langue").padding()) {
                    SettingsRow(icon: "textformat", title: "Langue")
                }
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
        }
    }

    // MARK: - About Section

    private var aboutSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("À PROPOS")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                NavigationLink(destination: AboutCellaView()) {
                    SettingsRow(icon: "cross", title: "Qu'est-ce que Cella ?")
                }
                Divider().padding(.leading, 52)
                NavigationLink(destination: Text("Romcal — Moteur liturgique libre").padding()) {
                    SettingsRow(icon: "gearshape.2", title: "Propulsé par Romcal")
                }
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
        }
    }
}

// MARK: - Explore Row

struct ExploreRow: View {
    let icon: String
    let title: String
    let subtitle: String
    let comingSoon: Bool

    var body: some View {
        HStack(spacing: 14) {
            Image(systemName: icon)
                .font(.system(size: 18))
                .foregroundStyle(Color.textSecondary)
                .frame(width: 28)

            VStack(alignment: .leading, spacing: 2) {
                Text(title)
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)

                Text(subtitle)
                    .font(CellaFont.caption(13))
                    .foregroundStyle(Color.textTertiary)
            }

            Spacer()

            if comingSoon {
                Text("Bientôt")
                    .font(CellaFont.caption(11))
                    .foregroundStyle(Color.textTertiary)
                    .padding(.horizontal, 10)
                    .padding(.vertical, 4)
                    .background(Color.parchmentWarm)
                    .clipShape(Capsule())
            }
        }
        .padding(.horizontal, CellaTheme.cardPadding)
        .padding(.vertical, 14)
    }
}

// MARK: - Settings Row

struct SettingsRow: View {
    let icon: String
    let title: String

    var body: some View {
        HStack(spacing: 14) {
            Image(systemName: icon)
                .font(.system(size: 17))
                .foregroundStyle(Color.textSecondary)
                .frame(width: 28)

            Text(title)
                .font(CellaFont.label(15))
                .foregroundStyle(Color.textPrimary)

            Spacer()

            Image(systemName: "chevron.right")
                .font(.system(size: 12))
                .foregroundStyle(Color.textTertiary)
        }
        .padding(.horizontal, CellaTheme.cardPadding)
        .padding(.vertical, 14)
    }
}

// MARK: - Preview

#Preview {
    MoreView()
}
