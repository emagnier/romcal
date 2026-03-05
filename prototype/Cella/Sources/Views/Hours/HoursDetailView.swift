import SwiftUI

// MARK: - Hours Detail View (Full Office Text)

struct HoursDetailView: View {
    let officeType: OfficeType
    let day: LiturgicalDay
    let sourceName: String
    let content: OfficeContent

    @State private var guidedMode = false
    @State private var currentSectionIndex = 0
    @Environment(\.dismiss) private var dismiss

    private var liturgicalColor: Color { day.color.color }

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                Rectangle()
                    .fill(liturgicalColor)
                    .frame(height: 3)

                VStack(spacing: 28) {
                    header
                    guidedModeToggle

                    ForEach(Array(content.sections.enumerated()), id: \.element.id) { index, section in
                        if !guidedMode || index <= currentSectionIndex {
                            officeSectionView(section, index: index)
                                .transition(.opacity.combined(with: .move(edge: .bottom)))
                        }
                    }

                    if guidedMode && currentSectionIndex < content.sections.count - 1 {
                        nextSectionButton
                    }
                }
                .padding(.horizontal, CellaTheme.horizontalPadding)
                .padding(.vertical, CellaTheme.verticalSpacing)
            }
        }
        .background(Color.parchmentWarm)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .principal) {
                Text(officeType.rawValue)
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
            }
        }
    }

    // MARK: - Header

    private var header: some View {
        VStack(spacing: 6) {
            Text("\(officeType.rawValue.uppercased()) — \(officeType.subtitle)")
                .font(CellaFont.label(12))
                .foregroundStyle(Color.textSecondary)
                .tracking(1)

            Text(day.title)
                .font(.system(size: 16, design: .serif))
                .foregroundStyle(Color.textPrimary)

            Text(sourceName)
                .font(CellaFont.caption(12))
                .foregroundStyle(Color.textTertiary)
        }
    }

    // MARK: - Guided Mode Toggle

    private var guidedModeToggle: some View {
        Button(action: {
            withAnimation(.easeInOut(duration: 0.3)) {
                guidedMode.toggle()
                if guidedMode {
                    currentSectionIndex = 0
                }
            }
        }) {
            HStack(spacing: 8) {
                Image(systemName: guidedMode ? "book.closed" : "book")
                    .font(.system(size: 13))
                Text(guidedMode ? "Mode continu" : "Prier pas à pas")
                    .font(CellaFont.caption(13))
            }
            .foregroundStyle(liturgicalColor)
            .padding(.horizontal, 16)
            .padding(.vertical, 8)
            .background(liturgicalColor.opacity(0.08))
            .clipShape(Capsule())
        }
        .buttonStyle(.plain)
    }

    // MARK: - Office Section

    private func officeSectionView(_ section: OfficeSection, index: Int) -> some View {
        VStack(alignment: .leading, spacing: 12) {
            // Section separator
            if index > 0 {
                HStack(spacing: 12) {
                    Rectangle()
                        .fill(Color.separator)
                        .frame(height: 0.5)
                    Text("✦")
                        .font(.system(size: 8))
                        .foregroundStyle(Color.separator)
                    Rectangle()
                        .fill(Color.separator)
                        .frame(height: 0.5)
                }
                .padding(.bottom, 4)
            }

            // Title
            HStack(alignment: .firstTextBaseline) {
                Text(section.title.uppercased())
                    .font(CellaFont.label(11))
                    .foregroundStyle(Color.textTertiary)
                    .tracking(1.5)

                if let subtitle = section.subtitle {
                    Text(subtitle)
                        .font(.system(size: 13, design: .serif).italic())
                        .foregroundStyle(Color.textTertiary)
                }
            }

            // Content
            if section.isAntiphon {
                antiphonView(section.content)
            } else if section.isResponse {
                responseView(section.content)
            } else {
                Text(section.content)
                    .font(.system(size: 15.5, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)
            }
        }
    }

    // MARK: - Antiphon View

    private func antiphonView(_ text: String) -> some View {
        HStack(alignment: .top, spacing: 10) {
            Text("Ant.")
                .font(.system(size: 13, weight: .medium, design: .serif))
                .foregroundStyle(liturgicalColor)
                .frame(width: 32, alignment: .leading)

            Text(text)
                .font(.system(size: 15, weight: .medium, design: .serif).italic())
                .foregroundStyle(Color.textPrimary)
                .lineSpacing(5)
        }
        .padding(14)
        .background(liturgicalColor.opacity(0.05))
        .clipShape(RoundedRectangle(cornerRadius: CellaTheme.smallRadius))
    }

    // MARK: - Response View

    private func responseView(_ text: String) -> some View {
        VStack(alignment: .leading, spacing: 4) {
            ForEach(text.components(separatedBy: "\n"), id: \.self) { line in
                if line.hasPrefix("℟") {
                    Text(line)
                        .font(.system(size: 15, weight: .medium, design: .serif))
                        .foregroundStyle(Color.textPrimary)
                        .lineSpacing(5)
                } else {
                    Text(line)
                        .font(.system(size: 15, design: .serif))
                        .foregroundStyle(Color.textSecondary)
                        .lineSpacing(5)
                }
            }
        }
    }

    // MARK: - Next Section Button (Guided Mode)

    private var nextSectionButton: some View {
        Button(action: {
            withAnimation(.easeInOut(duration: 0.4)) {
                currentSectionIndex += 1
            }
        }) {
            HStack(spacing: 8) {
                Text("Suite")
                    .font(CellaFont.label(14))
                Image(systemName: "chevron.down")
                    .font(.system(size: 12))
            }
            .foregroundStyle(.white)
            .padding(.horizontal, 28)
            .padding(.vertical, 12)
            .background(liturgicalColor)
            .clipShape(Capsule())
        }
        .buttonStyle(.plain)
        .padding(.top, 12)
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        HoursDetailView(
            officeType: .lauds,
            day: MockData.today,
            sourceName: "Liturgie des Heures (Église)",
            content: MockData.laudsContent
        )
    }
}
