import SwiftUI

// MARK: - Calendar & Location Settings

struct CalendarSettingsView: View {
    @State private var selectedRegionId: String = "europe"
    @State private var selectedCountryId: String = "france"
    @State private var selectedDioceseId: String = "france__paris"
    @State private var epiphanyOnSunday = true
    @State private var ascensionOnSunday = false
    @State private var corpusChristiOnSunday = true
    @State private var easterCalculation = 0  // 0 = Gregorian, 1 = Julian
    @State private var yearFrame = 0  // 0 = Civil, 1 = Liturgical

    private let regions = MockData.regions
    private let liturgicalColor: Color = MockData.today.color.color

    private var selectedRegion: CalendarRegion? {
        regions.first(where: { $0.id == selectedRegionId })
    }

    private var selectedCountry: CalendarCountry? {
        selectedRegion?.countries.first(where: { $0.id == selectedCountryId })
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                Rectangle()
                    .fill(liturgicalColor)
                    .frame(height: 2)

                VStack(spacing: CellaTheme.sectionSpacing) {
                    calendarSection
                    regionalOptionsSection
                    easterSection
                    yearFrameSection
                }
                .padding(.horizontal, CellaTheme.horizontalPadding)
                .padding(.vertical, CellaTheme.verticalSpacing)
            }
        }
        .background(Color.parchmentWarm)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .principal) {
                Text("Calendrier & Lieu")
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
            }
        }
    }

    // MARK: - Calendar Section

    private var calendarSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("CALENDRIER LITURGIQUE")
                .cellaSectionHeader()

            VStack(spacing: 14) {
                // Region
                SettingsPickerField(
                    label: "Région",
                    selection: $selectedRegionId,
                    options: regions.map { ($0.id, $0.name) }
                )

                // Country
                if let region = selectedRegion, !region.countries.isEmpty {
                    SettingsPickerField(
                        label: "Pays",
                        selection: $selectedCountryId,
                        options: region.countries.map { ($0.id, $0.name) }
                    )
                }

                // Diocese
                if let country = selectedCountry, !country.dioceses.isEmpty {
                    SettingsPickerField(
                        label: "Diocèse (optionnel)",
                        selection: $selectedDioceseId,
                        options: country.dioceses.map { ($0.id, $0.name) }
                    )
                }
            }
            .cellaCard()

            // Hierarchy explanation
            HStack(spacing: 6) {
                Image(systemName: "info.circle")
                    .font(.system(size: 12))
                Text("general_roman → \(selectedRegionId) → \(selectedCountryId) → \(selectedDioceseId)")
                    .lineLimit(1)
                    .minimumScaleFactor(0.7)
            }
            .font(CellaFont.caption(11))
            .foregroundStyle(Color.textTertiary)
        }
    }

    // MARK: - Regional Options

    private var regionalOptionsSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("OPTIONS RÉGIONALES")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                SettingsToggleRow(
                    title: "Épiphanie le dimanche",
                    subtitle: "Entre le 2 et le 8 janvier",
                    isOn: $epiphanyOnSunday
                )
                Divider().padding(.horizontal, CellaTheme.cardPadding)
                SettingsToggleRow(
                    title: "Ascension le dimanche",
                    subtitle: "7e dimanche de Pâques",
                    isOn: $ascensionOnSunday
                )
                Divider().padding(.horizontal, CellaTheme.cardPadding)
                SettingsToggleRow(
                    title: "Fête-Dieu le dimanche",
                    subtitle: "Dimanche après la Trinité",
                    isOn: $corpusChristiOnSunday
                )
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
        }
    }

    // MARK: - Easter Calculation

    private var easterSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("CALCUL DE PÂQUES")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                SettingsRadioRow(
                    title: "Grégorien (1583+)",
                    isSelected: easterCalculation == 0,
                    onSelect: { easterCalculation = 0 },
                    color: liturgicalColor
                )
                Divider().padding(.horizontal, CellaTheme.cardPadding)
                SettingsRadioRow(
                    title: "Julien (326+)",
                    isSelected: easterCalculation == 1,
                    onSelect: { easterCalculation = 1 },
                    color: liturgicalColor
                )
            }
            .background(Color.parchment)
            .clipShape(RoundedRectangle(cornerRadius: CellaTheme.cardRadius))
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(Color.separator, lineWidth: 0.5)
            )
        }
    }

    // MARK: - Year Frame

    private var yearFrameSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("CADRAGE DE L'ANNÉE")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                SettingsRadioRow(
                    title: "Civil (janvier → décembre)",
                    isSelected: yearFrame == 0,
                    onSelect: { yearFrame = 0 },
                    color: liturgicalColor
                )
                Divider().padding(.horizontal, CellaTheme.cardPadding)
                SettingsRadioRow(
                    title: "Liturgique (Avent → Christ Roi)",
                    isSelected: yearFrame == 1,
                    onSelect: { yearFrame = 1 },
                    color: liturgicalColor
                )
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

// MARK: - Settings Picker Field

struct SettingsPickerField: View {
    let label: String
    @Binding var selection: String
    let options: [(String, String)] // (id, name)

    var body: some View {
        VStack(alignment: .leading, spacing: 6) {
            Text(label)
                .font(CellaFont.caption(13))
                .foregroundStyle(Color.textSecondary)

            Menu {
                ForEach(options, id: \.0) { option in
                    Button(action: { selection = option.0 }) {
                        HStack {
                            Text(option.1)
                            if selection == option.0 {
                                Image(systemName: "checkmark")
                            }
                        }
                    }
                }
            } label: {
                HStack {
                    Text(options.first(where: { $0.0 == selection })?.1 ?? "Sélectionner…")
                        .font(CellaFont.labelLight(15))
                        .foregroundStyle(Color.textPrimary)
                    Spacer()
                    Image(systemName: "chevron.down")
                        .font(.system(size: 12))
                        .foregroundStyle(Color.textTertiary)
                }
                .padding(.horizontal, 14)
                .padding(.vertical, 10)
                .background(Color.parchmentWarm)
                .clipShape(RoundedRectangle(cornerRadius: CellaTheme.smallRadius))
                .overlay(
                    RoundedRectangle(cornerRadius: CellaTheme.smallRadius)
                        .stroke(Color.separator, lineWidth: 0.5)
                )
            }
        }
    }
}

// MARK: - Settings Toggle Row

struct SettingsToggleRow: View {
    let title: String
    let subtitle: String
    @Binding var isOn: Bool

    var body: some View {
        HStack {
            VStack(alignment: .leading, spacing: 3) {
                Text(title)
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
                Text(subtitle)
                    .font(CellaFont.caption(13))
                    .foregroundStyle(Color.textTertiary)
            }
            Spacer()
            Toggle("", isOn: $isOn)
                .labelsHidden()
                .tint(Color.accentGold)
        }
        .padding(.horizontal, CellaTheme.cardPadding)
        .padding(.vertical, 12)
    }
}

// MARK: - Settings Radio Row

struct SettingsRadioRow: View {
    let title: String
    let isSelected: Bool
    let onSelect: () -> Void
    let color: Color

    var body: some View {
        Button(action: onSelect) {
            HStack(spacing: 14) {
                ZStack {
                    Circle()
                        .stroke(isSelected ? color : Color.textTertiary, lineWidth: 1.5)
                        .frame(width: 20, height: 20)
                    if isSelected {
                        Circle()
                            .fill(color)
                            .frame(width: 12, height: 12)
                    }
                }

                Text(title)
                    .font(CellaFont.labelLight(15))
                    .foregroundStyle(isSelected ? Color.textPrimary : Color.textSecondary)

                Spacer()
            }
            .padding(.horizontal, CellaTheme.cardPadding)
            .padding(.vertical, 12)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        CalendarSettingsView()
    }
}
