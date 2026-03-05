import SwiftUI

// MARK: - Hours Tab (List of Offices)

struct HoursListView: View {
    @State private var selectedSource: OfficeSource = MockData.officeSources[0]
    @State private var showSourcePicker = false
    @State private var selectedOffice: OfficeType? = nil

    private let day = MockData.today
    private let sources = MockData.officeSources

    private var liturgicalColor: Color { day.color.color }

    private var suggestedOffice: OfficeType {
        let hour = Calendar.current.component(.hour, from: Date())
        return OfficeType.allCases.first(where: { $0.suggestedHourRange.contains(hour) }) ?? .vespers
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 0) {
                    Rectangle()
                        .fill(liturgicalColor)
                        .frame(height: 3)

                    VStack(spacing: CellaTheme.sectionSpacing) {
                        header
                        sourceSelector
                        officesList
                    }
                    .padding(.horizontal, CellaTheme.horizontalPadding)
                    .padding(.vertical, CellaTheme.verticalSpacing)
                }
            }
            .background(Color.parchmentWarm)
            .navigationBarTitleDisplayMode(.inline)
            .navigationDestination(item: $selectedOffice) { office in
                HoursDetailView(
                    officeType: office,
                    day: day,
                    sourceName: selectedSource.name,
                    content: MockData.laudsContent
                )
            }
        }
    }

    // MARK: - Header

    private var header: some View {
        VStack(spacing: 6) {
            Text("LITURGIE DES HEURES")
                .font(CellaFont.label(12))
                .foregroundStyle(Color.textSecondary)
                .tracking(1.5)

            Text("\(dayOfWeekString) — \(day.title)")
                .font(.system(size: 16, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .multilineTextAlignment(.center)

            Text("Psautier : Semaine \(day.psalterWeek.romanNumeral)")
                .font(CellaFont.caption(13))
                .foregroundStyle(Color.textTertiary)
        }
    }

    // MARK: - Source Selector

    private var sourceSelector: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text("SOURCE")
                .cellaSectionHeader()

            Button(action: { showSourcePicker.toggle() }) {
                HStack {
                    Text(selectedSource.name)
                        .font(CellaFont.label(14))
                        .foregroundStyle(Color.textPrimary)
                    Spacer()
                    Image(systemName: "chevron.down")
                        .font(.system(size: 12))
                        .foregroundStyle(Color.textTertiary)
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 12)
                .background(Color.parchment)
                .clipShape(RoundedRectangle(cornerRadius: CellaTheme.smallRadius))
                .overlay(
                    RoundedRectangle(cornerRadius: CellaTheme.smallRadius)
                        .stroke(Color.separator, lineWidth: 0.5)
                )
            }
            .buttonStyle(.plain)
            .sheet(isPresented: $showSourcePicker) {
                sourcePickerSheet
            }
        }
    }

    private var sourcePickerSheet: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("Source de l'office")
                .font(.system(size: 18, weight: .semibold, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .padding(.bottom, 8)

            ForEach(sources) { source in
                Button(action: {
                    selectedSource = source
                    showSourcePicker = false
                }) {
                    HStack(spacing: 14) {
                        ZStack {
                            Circle()
                                .stroke(selectedSource.id == source.id ? liturgicalColor : Color.textTertiary, lineWidth: 1.5)
                                .frame(width: 20, height: 20)
                            if selectedSource.id == source.id {
                                Circle()
                                    .fill(liturgicalColor)
                                    .frame(width: 12, height: 12)
                            }
                        }

                        Text(source.name)
                            .font(CellaFont.labelLight(15))
                            .foregroundStyle(selectedSource.id == source.id ? Color.textPrimary : Color.textSecondary)

                        Spacer()
                    }
                    .padding(.vertical, 6)
                }
                .buttonStyle(.plain)
            }

            Spacer()
        }
        .padding(CellaTheme.horizontalPadding)
        .padding(.top, 24)
        .presentationDetents([.medium])
        .presentationDragIndicator(.visible)
        .background(Color.parchment)
    }

    // MARK: - Offices List

    private var officesList: some View {
        VStack(spacing: 12) {
            // Major offices
            OfficeCard(
                office: .officeOfReadings,
                isSuggested: suggestedOffice == .officeOfReadings,
                liturgicalColor: liturgicalColor,
                onTap: { selectedOffice = .officeOfReadings }
            )

            OfficeCard(
                office: .lauds,
                isSuggested: suggestedOffice == .lauds,
                liturgicalColor: liturgicalColor,
                onTap: { selectedOffice = .lauds }
            )

            // Mid-day offices (compact row)
            HStack(spacing: 10) {
                ForEach([OfficeType.terce, .sext, .none_], id: \.self) { office in
                    MidDayOfficeCard(
                        office: office,
                        isSuggested: suggestedOffice == office,
                        liturgicalColor: liturgicalColor,
                        onTap: { selectedOffice = office }
                    )
                }
            }

            Text("Prière du milieu du jour")
                .font(CellaFont.caption(12))
                .foregroundStyle(Color.textTertiary)
                .padding(.bottom, 4)

            OfficeCard(
                office: .vespers,
                isSuggested: suggestedOffice == .vespers,
                liturgicalColor: liturgicalColor,
                onTap: { selectedOffice = .vespers }
            )

            OfficeCard(
                office: .compline,
                isSuggested: suggestedOffice == .compline,
                liturgicalColor: liturgicalColor,
                onTap: { selectedOffice = .compline }
            )
        }
    }

    // MARK: - Helpers

    private var dayOfWeekString: String {
        let formatter = DateFormatter()
        formatter.locale = Locale(identifier: "fr_FR")
        formatter.dateFormat = "EEEE"
        let result = formatter.string(from: day.date)
        return result.prefix(1).uppercased() + result.dropFirst()
    }
}

// MARK: - Office Card (Major)

struct OfficeCard: View {
    let office: OfficeType
    let isSuggested: Bool
    let liturgicalColor: Color
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            HStack(spacing: 14) {
                Image(systemName: office.icon)
                    .font(.system(size: 24))
                    .foregroundStyle(isSuggested ? liturgicalColor : Color.textSecondary)
                    .frame(width: 36)

                VStack(alignment: .leading, spacing: 4) {
                    HStack(spacing: 8) {
                        Text(office.rawValue.uppercased())
                            .font(CellaFont.label(13))
                            .foregroundStyle(isSuggested ? liturgicalColor : Color.textPrimary)
                            .tracking(0.5)

                        if isSuggested {
                            Circle()
                                .fill(liturgicalColor)
                                .frame(width: 6, height: 6)
                        }
                    }

                    Text(office.subtitle)
                        .font(CellaFont.caption(13))
                        .foregroundStyle(Color.textTertiary)

                    Text(office.components)
                        .font(CellaFont.caption(12))
                        .foregroundStyle(Color.textTertiary.opacity(0.7))
                }

                Spacer()

                Image(systemName: "chevron.right")
                    .font(.system(size: 12))
                    .foregroundStyle(Color.textTertiary)
            }
            .cellaCard(liturgicalColor: isSuggested ? liturgicalColor : nil)
        }
        .buttonStyle(.plain)
    }
}

// MARK: - Mid-Day Office Card (Compact)

struct MidDayOfficeCard: View {
    let office: OfficeType
    let isSuggested: Bool
    let liturgicalColor: Color
    let onTap: () -> Void

    var body: some View {
        Button(action: onTap) {
            VStack(spacing: 6) {
                Image(systemName: office.icon)
                    .font(.system(size: 20))
                    .foregroundStyle(isSuggested ? liturgicalColor : Color.textSecondary)

                Text(office.rawValue.uppercased())
                    .font(CellaFont.label(11))
                    .foregroundStyle(isSuggested ? liturgicalColor : Color.textPrimary)
                    .tracking(0.5)

                Text(hourLabel)
                    .font(CellaFont.caption(11))
                    .foregroundStyle(Color.textTertiary)

                if isSuggested {
                    Circle()
                        .fill(liturgicalColor)
                        .frame(width: 5, height: 5)
                } else {
                    Spacer().frame(height: 5)
                }
            }
            .frame(maxWidth: .infinity)
            .padding(.vertical, 14)
            .background(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .fill(isSuggested ? liturgicalColor.opacity(0.06) : Color.parchment)
            )
            .overlay(
                RoundedRectangle(cornerRadius: CellaTheme.cardRadius)
                    .stroke(isSuggested ? liturgicalColor.opacity(0.3) : Color.separator, lineWidth: 0.5)
            )
        }
        .buttonStyle(.plain)
    }

    private var hourLabel: String {
        switch office {
        case .terce: return "9h"
        case .sext: return "12h"
        case .none_: return "15h"
        default: return ""
        }
    }
}

// MARK: - OfficeType Hashable

extension OfficeType: Hashable {}

// MARK: - Preview

#Preview {
    HoursListView()
}
