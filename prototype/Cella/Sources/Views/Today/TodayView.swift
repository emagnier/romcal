import SwiftUI

// MARK: - Today Tab

struct TodayView: View {
    @State private var selectedCelebrationId: String
    @State private var dayOffset: Int = 0
    @State private var showCalendar = false
    @State private var prayedOffices: Set<String> = ["officeOfReadings", "lauds"]

    private let day: LiturgicalDay

    init(day: LiturgicalDay = MockData.today) {
        self.day = day
        self._selectedCelebrationId = State(initialValue: day.celebrations.first?.id ?? "")
    }

    private var selectedCelebration: Celebration {
        day.celebrations.first(where: { $0.id == selectedCelebrationId }) ?? day.celebrations[0]
    }

    private var currentLiturgicalColor: Color {
        selectedCelebration.color.color
    }

    private var suggestedOffice: OfficeType {
        let hour = Calendar.current.component(.hour, from: Date())
        return OfficeType.allCases.first(where: { $0.suggestedHourRange.contains(hour) }) ?? .vespers
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 0) {
                    // Liturgical color bar at top
                    Rectangle()
                        .fill(currentLiturgicalColor)
                        .frame(height: 3)

                    VStack(spacing: CellaTheme.sectionSpacing) {
                        dayHeader
                        suggestedOfficeCard
                        celebrationsSection
                        collectSection
                        martyrologySection
                        allOfficesGrid
                    }
                    .padding(.horizontal, CellaTheme.horizontalPadding)
                    .padding(.vertical, CellaTheme.verticalSpacing)
                }
            }
            .background(Color.parchmentWarm)
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    // MARK: - Day Header

    private var dayHeader: some View {
        VStack(spacing: 8) {
            // Date navigation
            HStack {
                Button(action: {}) {
                    HStack(spacing: 4) {
                        Image(systemName: "chevron.left")
                        Text("4 mars")
                    }
                    .font(CellaFont.caption(13))
                    .foregroundStyle(Color.textSecondary)
                }

                Spacer()

                Button(action: {}) {
                    HStack(spacing: 4) {
                        Text("6 mars")
                        Image(systemName: "chevron.right")
                    }
                    .font(CellaFont.caption(13))
                    .foregroundStyle(Color.textSecondary)
                }
            }

            // Main title
            Text(day.title)
                .font(.system(size: 24, weight: .semibold, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .multilineTextAlignment(.center)
                .padding(.top, 4)

            // Subtitle with liturgical color indicator
            HStack(spacing: 8) {
                RoundedRectangle(cornerRadius: 2)
                    .fill(currentLiturgicalColor)
                    .frame(width: CellaTheme.liturgicalBarWidth, height: 16)

                Text(day.subtitle)
                    .font(CellaFont.labelLight(14))
                    .foregroundStyle(Color.textSecondary)
            }

            Text("Semaine du Psautier : \(day.psalterWeek.romanNumeral)")
                .font(CellaFont.caption(12))
                .foregroundStyle(Color.textTertiary)
        }
        .padding(.bottom, 8)
    }

    // MARK: - Suggested Office Card

    private var suggestedOfficeCard: some View {
        VStack(alignment: .leading, spacing: 0) {
            HStack {
                Text("OFFICE SUGGÉRÉ")
                    .cellaSectionHeader()
                Spacer()
                Text(timeString)
                    .font(CellaFont.label(12))
                    .foregroundStyle(Color.textTertiary)
            }
            .padding(.bottom, 12)

            HStack(spacing: 14) {
                Image(systemName: suggestedOffice.icon)
                    .font(.system(size: 22))
                    .foregroundStyle(currentLiturgicalColor)
                    .frame(width: 32)

                VStack(alignment: .leading, spacing: 4) {
                    Text("\(suggestedOffice.rawValue) — \(suggestedOffice.subtitle)")
                        .font(CellaFont.label(15))
                        .foregroundStyle(Color.textPrimary)

                    Text("« Seigneur, apprends-nous à compter nos jours… »")
                        .font(.system(size: 14, design: .serif).italic())
                        .foregroundStyle(Color.textSecondary)
                        .lineLimit(2)
                }

                Spacer()

                Text("Prier")
                    .font(CellaFont.label(13))
                    .foregroundStyle(currentLiturgicalColor)
                + Text(" →")
                    .font(CellaFont.label(13))
                    .foregroundStyle(currentLiturgicalColor)
            }
            .cellaCard(liturgicalColor: currentLiturgicalColor)
        }
    }

    // MARK: - Celebrations Section

    private var celebrationsSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("CÉLÉBRATIONS DU JOUR")
                .cellaSectionHeader()

            ForEach(day.celebrations) { celebration in
                CelebrationRow(
                    celebration: celebration,
                    isSelected: selectedCelebrationId == celebration.id,
                    liturgicalColor: celebration.color.color,
                    onSelect: {
                        withAnimation(.easeInOut(duration: 0.25)) {
                            selectedCelebrationId = celebration.id
                        }
                    }
                )
            }
        }
    }

    // MARK: - Collect Section

    private var collectSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("ORAISON DU JOUR")
                .cellaSectionHeader()

            VStack(alignment: .leading, spacing: 16) {
                Text("Collecte")
                    .font(CellaFont.label(13))
                    .foregroundStyle(Color.textSecondary)

                Text(day.collect)
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)

                HStack(spacing: 10) {
                    OraisionPillButton(title: "Sur les offrandes", text: day.prayerOverOfferings)
                    OraisionPillButton(title: "Après la communion", text: day.prayerAfterCommunion)
                }
            }
            .cellaCard()
        }
    }

    // MARK: - Martyrology Section

    @ViewBuilder
    private var martyrologySection: some View {
        if let entry = day.martyrologyEntry {
            VStack(alignment: .leading, spacing: 12) {
                Text("MÉMOIRE DU JOUR")
                    .cellaSectionHeader()

                VStack(alignment: .leading, spacing: 12) {
                    HStack(alignment: .top) {
                        VStack(alignment: .leading, spacing: 4) {
                            Text(entry.fullname)
                                .font(.system(size: 18, weight: .semibold, design: .serif))
                                .foregroundStyle(Color.textPrimary)

                            if let titles = entry.titles.first {
                                HStack(spacing: 6) {
                                    Text(titles)
                                        .font(CellaFont.caption(13))
                                        .foregroundStyle(Color.textSecondary)
                                    if let death = entry.dateOfDeath {
                                        Text("(✝ \(death))")
                                            .font(CellaFont.caption(13))
                                            .foregroundStyle(Color.textTertiary)
                                    }
                                }
                            }
                        }
                        Spacer()
                    }

                    Text(entry.shortBio)
                        .font(.system(size: 15, design: .serif))
                        .foregroundStyle(Color.textSecondary)
                        .lineSpacing(5)
                        .lineLimit(4)

                    HStack {
                        Spacer()
                        Text("Lire plus →")
                            .font(CellaFont.label(13))
                            .foregroundStyle(currentLiturgicalColor)
                    }
                }
                .cellaCard(liturgicalColor: currentLiturgicalColor)
            }
        }
    }

    // MARK: - All Offices Grid

    private var allOfficesGrid: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("TOUS LES OFFICES")
                .cellaSectionHeader()

            LazyVGrid(columns: [
                GridItem(.flexible(), spacing: 10),
                GridItem(.flexible(), spacing: 10),
                GridItem(.flexible(), spacing: 10),
                GridItem(.flexible(), spacing: 10)
            ], spacing: 10) {
                ForEach(OfficeType.allCases) { office in
                    OfficeGridCell(
                        office: office,
                        isPrayed: prayedOffices.contains(office.id),
                        isSuggested: office == suggestedOffice,
                        liturgicalColor: currentLiturgicalColor
                    )
                }
            }

            // Legend
            HStack(spacing: 16) {
                HStack(spacing: 4) {
                    Image(systemName: "checkmark")
                        .font(.system(size: 9))
                        .foregroundStyle(Color.textTertiary)
                    Text("prié")
                        .font(CellaFont.caption(11))
                        .foregroundStyle(Color.textTertiary)
                }
                HStack(spacing: 4) {
                    Circle()
                        .fill(currentLiturgicalColor)
                        .frame(width: 6, height: 6)
                    Text("suggéré")
                        .font(CellaFont.caption(11))
                        .foregroundStyle(Color.textTertiary)
                }
            }
            .padding(.top, 4)
        }
    }

    // MARK: - Helpers

    private var timeString: String {
        let formatter = DateFormatter()
        formatter.dateFormat = "HH:mm"
        return formatter.string(from: Date())
    }
}

// MARK: - Celebration Row

struct CelebrationRow: View {
    let celebration: Celebration
    let isSelected: Bool
    let liturgicalColor: Color
    let onSelect: () -> Void

    var body: some View {
        Button(action: onSelect) {
            HStack(spacing: 14) {
                // Radio button
                ZStack {
                    Circle()
                        .stroke(isSelected ? liturgicalColor : Color.textTertiary, lineWidth: 1.5)
                        .frame(width: 20, height: 20)
                    if isSelected {
                        Circle()
                            .fill(liturgicalColor)
                            .frame(width: 12, height: 12)
                    }
                }

                VStack(alignment: .leading, spacing: 3) {
                    Text(celebration.fullname)
                        .font(.system(size: 15, weight: isSelected ? .medium : .regular, design: .serif))
                        .foregroundStyle(isSelected ? Color.textPrimary : Color.textSecondary)

                    Text(celebration.rankLabel)
                        .font(CellaFont.caption(12))
                        .foregroundStyle(Color.textTertiary)
                }

                Spacer()

                if celebration.isOptional {
                    Image(systemName: "info.circle")
                        .font(.system(size: 14))
                        .foregroundStyle(Color.textTertiary)
                }
            }
            .cellaCard(liturgicalColor: isSelected ? liturgicalColor : nil)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Oraison Pill Button

struct OraisionPillButton: View {
    let title: String
    let text: String
    @State private var showSheet = false

    var body: some View {
        Button(action: { showSheet = true }) {
            Text(title)
                .font(CellaFont.caption(12))
                .foregroundStyle(Color.textSecondary)
                .padding(.horizontal, 14)
                .padding(.vertical, 8)
                .background(Color.parchmentWarm)
                .clipShape(Capsule())
                .overlay(
                    Capsule().stroke(Color.separator, lineWidth: 0.5)
                )
        }
        .buttonStyle(.plain)
        .sheet(isPresented: $showSheet) {
            VStack(alignment: .leading, spacing: 16) {
                Text(title)
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)

                Text(text)
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)

                Spacer()
            }
            .padding(CellaTheme.horizontalPadding)
            .padding(.top, 24)
            .presentationDetents([.medium])
            .presentationDragIndicator(.visible)
            .background(Color.parchment)
        }
    }
}

// MARK: - Office Grid Cell

struct OfficeGridCell: View {
    let office: OfficeType
    let isPrayed: Bool
    let isSuggested: Bool
    let liturgicalColor: Color

    var body: some View {
        VStack(spacing: 6) {
            Text(office == .officeOfReadings ? "Vigiles" : office.rawValue.components(separatedBy: " ").first ?? office.rawValue)
                .font(CellaFont.label(12))
                .foregroundStyle(isSuggested ? liturgicalColor : Color.textPrimary)
                .lineLimit(1)
                .minimumScaleFactor(0.8)

            ZStack {
                if isPrayed {
                    Image(systemName: "checkmark")
                        .font(.system(size: 9, weight: .bold))
                        .foregroundStyle(Color.textTertiary)
                } else if isSuggested {
                    Circle()
                        .fill(liturgicalColor)
                        .frame(width: 6, height: 6)
                }
            }
            .frame(height: 10)
        }
        .frame(maxWidth: .infinity)
        .padding(.vertical, 12)
        .background(
            RoundedRectangle(cornerRadius: CellaTheme.smallRadius)
                .fill(isSuggested ? liturgicalColor.opacity(0.08) : Color.parchment)
        )
        .overlay(
            RoundedRectangle(cornerRadius: CellaTheme.smallRadius)
                .stroke(isSuggested ? liturgicalColor.opacity(0.3) : Color.separator, lineWidth: 0.5)
        )
    }
}

// MARK: - Roman Numeral Helper

extension Int {
    var romanNumeral: String {
        switch self {
        case 1: return "I"
        case 2: return "II"
        case 3: return "III"
        case 4: return "IV"
        default: return "\(self)"
        }
    }
}

// MARK: - Preview

#Preview {
    TodayView()
}
