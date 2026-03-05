import SwiftUI

// MARK: - Appearance Settings

struct AppearanceSettingsView: View {
    @State private var colorScheme: Int = 0  // 0=auto, 1=light, 2=dark
    @State private var textSize: Double = 16
    @State private var liturgicalColorImmersive = false

    private let liturgicalColor: Color = MockData.today.color.color

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                Rectangle()
                    .fill(liturgicalColor)
                    .frame(height: 2)

                VStack(spacing: CellaTheme.sectionSpacing) {
                    themeSection
                    textSizeSection
                    liturgicalColorSection
                }
                .padding(.horizontal, CellaTheme.horizontalPadding)
                .padding(.vertical, CellaTheme.verticalSpacing)
            }
        }
        .background(Color.parchmentWarm)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .principal) {
                Text("Apparence")
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
            }
        }
    }

    // MARK: - Theme

    private var themeSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("THÈME")
                .cellaSectionHeader()

            HStack(spacing: 12) {
                ThemeCard(
                    title: "Auto",
                    icon: "circle.lefthalf.filled",
                    isSelected: colorScheme == 0,
                    color: liturgicalColor,
                    onTap: { colorScheme = 0 }
                )
                ThemeCard(
                    title: "Clair",
                    icon: "sun.max",
                    isSelected: colorScheme == 1,
                    color: liturgicalColor,
                    onTap: { colorScheme = 1 }
                )
                ThemeCard(
                    title: "Sombre",
                    icon: "moon",
                    isSelected: colorScheme == 2,
                    color: liturgicalColor,
                    onTap: { colorScheme = 2 }
                )
            }
        }
    }

    // MARK: - Text Size

    private var textSizeSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("TAILLE DU TEXTE")
                .cellaSectionHeader()

            VStack(spacing: 12) {
                // Preview
                Text("« Demandez, on vous donnera ; cherchez, vous trouverez ; frappez, on vous ouvrira. »")
                    .font(.system(size: textSize, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(textSize * 0.35)
                    .padding(.bottom, 4)

                HStack(spacing: 12) {
                    Text("A")
                        .font(.system(size: 13, design: .serif))
                        .foregroundStyle(Color.textTertiary)

                    Slider(value: $textSize, in: 13...22, step: 1)
                        .tint(liturgicalColor)

                    Text("A")
                        .font(.system(size: 22, design: .serif))
                        .foregroundStyle(Color.textTertiary)
                }
            }
            .cellaCard()
        }
    }

    // MARK: - Liturgical Color Immersive

    private var liturgicalColorSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("COULEUR LITURGIQUE")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                SettingsToggleRow(
                    title: "Fond immersif",
                    subtitle: "Teinte légère de la couleur liturgique du jour",
                    isOn: $liturgicalColorImmersive
                )
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )

            // Preview of liturgical colors
            HStack(spacing: 8) {
                ForEach([LiturgicalColor.violet, .white, .red, .green, .rose], id: \.self) { color in
                    VStack(spacing: 4) {
                        Circle()
                            .fill(color.color)
                            .frame(width: 24, height: 24)
                        Text(color.label)
                            .font(CellaFont.caption(10))
                            .foregroundStyle(Color.textTertiary)
                    }
                }
            }
            .frame(maxWidth: .infinity)
            .padding(.top, 4)
        }
    }
}

// MARK: - Theme Card

struct ThemeCard: View {
    let title: String
    let icon: String
    let isSelected: Bool
    let color: Color
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            VStack(spacing: 10) {
                // Mini preview
                RoundedRectangle(cornerRadius: 8)
                    .fill(previewColor)
                    .frame(height: 48)
                    .overlay(
                        Image(systemName: icon)
                            .font(.system(size: 18))
                            .foregroundStyle(previewIconColor)
                    )
                    .overlay(
                        RoundedRectangle(cornerRadius: 8)
                            .stroke(isSelected ? color : Color.separator, lineWidth: isSelected ? 2 : 0.5)
                    )

                Text(title)
                    .font(CellaFont.caption(12))
                    .foregroundStyle(isSelected ? Color.textPrimary : Color.textSecondary)
            }
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(.plain)
    }

    private var previewColor: Color {
        switch title {
        case "Sombre": return Color.inkDark
        case "Clair": return Color.parchment
        default: return Color.parchmentWarm
        }
    }

    private var previewIconColor: Color {
        switch title {
        case "Sombre": return Color.parchment
        default: return Color.textSecondary
        }
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        AppearanceSettingsView()
    }
}
