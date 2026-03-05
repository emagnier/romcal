import SwiftUI

// MARK: - Readings Tab

struct ReadingsView: View {
    @State private var selectedFormularyIndex = 0
    @State private var showFormularyPicker = false

    private let day = MockData.today
    private let formularies = MockData.todayFormularies
    private let commentary = MockData.commentary

    private var currentFormulary: MassFormulary {
        formularies[selectedFormularyIndex]
    }

    private var liturgicalColor: Color {
        day.color.color
    }

    var body: some View {
        NavigationStack {
            ScrollView {
                VStack(spacing: 0) {
                    // Liturgical color bar
                    Rectangle()
                        .fill(liturgicalColor)
                        .frame(height: 3)

                    VStack(spacing: CellaTheme.sectionSpacing) {
                        header
                        formularyPicker
                        readingsContent
                        commentarySection
                    }
                    .padding(.horizontal, CellaTheme.horizontalPadding)
                    .padding(.vertical, CellaTheme.verticalSpacing)
                }
            }
            .background(Color.parchmentWarm)
            .navigationBarTitleDisplayMode(.inline)
        }
    }

    // MARK: - Header

    private var header: some View {
        VStack(spacing: 6) {
            Text(day.title.uppercased())
                .font(CellaFont.label(12))
                .foregroundStyle(Color.textSecondary)
                .tracking(1)

            Text("Année \(day.sundayCycle) — Cycle \(day.weekdayCycle)")
                .font(CellaFont.caption(13))
                .foregroundStyle(Color.textTertiary)
        }
    }

    // MARK: - Formulary Picker

    @ViewBuilder
    private var formularyPicker: some View {
        if formularies.count > 1 {
            Button(action: { showFormularyPicker.toggle() }) {
                HStack {
                    Text(currentFormulary.name)
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
        }
    }

    // MARK: - Readings Content

    private var readingsContent: some View {
        VStack(spacing: 0) {
            ForEach(Array(currentFormulary.readings.enumerated()), id: \.element.id) { index, reading in
                if index > 0 {
                    readingSeparator
                }
                ReadingSection(reading: reading, liturgicalColor: liturgicalColor)
            }
        }
    }

    private var readingSeparator: some View {
        HStack {
            Rectangle()
                .fill(Color.separator)
                .frame(height: 0.5)
        }
        .padding(.vertical, 24)
    }

    // MARK: - Commentary Section

    private var commentarySection: some View {
        VStack(alignment: .leading, spacing: 12) {
            readingSeparator

            Text("COMMENTAIRE")
                .cellaSectionHeader()

            VStack(alignment: .leading, spacing: 14) {
                HStack(spacing: 10) {
                    Text("✝")
                        .font(.system(size: 16))
                        .foregroundStyle(liturgicalColor)

                    VStack(alignment: .leading, spacing: 2) {
                        Text(commentary.author)
                            .font(.system(size: 16, weight: .semibold, design: .serif))
                            .foregroundStyle(Color.textPrimary)

                        Text(commentary.source)
                            .font(.system(size: 14, design: .serif).italic())
                            .foregroundStyle(Color.textSecondary)
                    }
                }

                Text(commentary.text)
                    .font(.system(size: 15.5, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)

                HStack {
                    Text("Source : \(commentary.sourceType)")
                        .font(CellaFont.caption(12))
                        .foregroundStyle(Color.textTertiary)

                    Spacer()

                    Image(systemName: "info.circle")
                        .font(.system(size: 13))
                        .foregroundStyle(Color.textTertiary)
                }
            }
            .cellaCard(liturgicalColor: liturgicalColor)
        }
    }
}

// MARK: - Reading Section

struct ReadingSection: View {
    let reading: MassReading
    let liturgicalColor: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            // Section header with type
            HStack {
                Rectangle()
                    .fill(Color.separator)
                    .frame(height: 0.5)
                    .frame(maxWidth: 20)

                Text(reading.type.rawValue)
                    .font(CellaFont.label(11))
                    .foregroundStyle(Color.textTertiary)
                    .tracking(1)

                Rectangle()
                    .fill(Color.separator)
                    .frame(height: 0.5)

                // Alternative indicator
                if let index = reading.alternativeIndex, let count = reading.alternativeCount {
                    HStack(spacing: 4) {
                        Text("\(index + 1)/\(count)")
                            .font(CellaFont.caption(11))
                            .foregroundStyle(Color.textTertiary)

                        VStack(spacing: 2) {
                            Image(systemName: "chevron.up")
                                .font(.system(size: 8))
                            Image(systemName: "chevron.down")
                                .font(.system(size: 8))
                        }
                        .foregroundStyle(Color.textTertiary)
                    }
                }
            }

            // Reference
            if reading.type == .gospel {
                HStack(spacing: 6) {
                    Text("✛")
                        .font(.system(size: 12))
                        .foregroundStyle(liturgicalColor)
                    Text("Évangile de Jésus Christ selon saint Matthieu")
                        .font(.system(size: 14, design: .serif).italic())
                        .foregroundStyle(Color.textSecondary)
                }
            }

            Text(reading.reference)
                .font(CellaFont.label(13))
                .foregroundStyle(Color.textSecondary)

            // Psalm response
            if let response = reading.response {
                HStack(spacing: 6) {
                    Text("℟")
                        .font(.system(size: 18, weight: .bold, design: .serif))
                        .foregroundStyle(liturgicalColor)

                    Text(response)
                        .font(.system(size: 15, weight: .medium, design: .serif).italic())
                        .foregroundStyle(Color.textPrimary)
                }
                .padding(.vertical, 4)
            }

            // Main text with lettrine for first reading and gospel
            if reading.type == .firstReading || reading.type == .gospel {
                LettrineText(text: reading.text, color: liturgicalColor)
            } else {
                Text(reading.text)
                    .font(.system(size: 15.5, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)
            }

            // Closing formula
            closingFormula(for: reading.type)
        }
    }

    @ViewBuilder
    private func closingFormula(for type: MassReading.ReadingType) -> some View {
        switch type {
        case .firstReading, .secondReading:
            Text("— Parole du Seigneur.")
                .font(.system(size: 14, weight: .medium, design: .serif))
                .foregroundStyle(Color.textSecondary)
                .padding(.top, 4)
        case .gospel:
            Text("— Acclamons la Parole de Dieu.")
                .font(.system(size: 14, weight: .medium, design: .serif))
                .foregroundStyle(Color.textSecondary)
                .padding(.top, 4)
        default:
            EmptyView()
        }
    }
}

// MARK: - Lettrine Text

struct LettrineText: View {
    let text: String
    let color: Color

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            if text.count > 1 {
                HStack(alignment: .top, spacing: 4) {
                    // Lettrine
                    Text(String(text.prefix(1)))
                        .font(.system(size: 52, weight: .bold, design: .serif))
                        .foregroundStyle(color)
                        .baselineOffset(-12)
                        .padding(.trailing, 2)

                    // First part of text
                    Text(String(text.dropFirst()))
                        .font(.system(size: 15.5, design: .serif))
                        .foregroundStyle(Color.textPrimary)
                        .lineSpacing(6)
                }
            } else {
                Text(text)
                    .font(.system(size: 15.5, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)
            }
        }
    }
}

// MARK: - Preview

#Preview {
    ReadingsView()
}
