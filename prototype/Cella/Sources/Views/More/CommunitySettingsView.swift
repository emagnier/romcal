import SwiftUI

// MARK: - Community Settings

struct CommunitySettingsView: View {
    @State private var selectedCommunityId: String = ""
    @State private var selectedProvinceId: String = ""
    @State private var defaultOffice: Int = 0  // 0 = Roman, 1 = Community
    @State private var showCommunityPicker = false

    private let communities = MockData.communities
    private let liturgicalColor: Color = MockData.today.color.color

    private var selectedCommunity: ReligiousCommunity? {
        communities.first(where: { $0.id == selectedCommunityId })
    }

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                Rectangle()
                    .fill(liturgicalColor)
                    .frame(height: 2)

                VStack(spacing: CellaTheme.sectionSpacing) {
                    introText
                    communitySection
                    provinceSection
                    officeDefaultSection
                }
                .padding(.horizontal, CellaTheme.horizontalPadding)
                .padding(.vertical, CellaTheme.verticalSpacing)
            }
        }
        .background(Color.parchmentWarm)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .principal) {
                Text("Communauté religieuse")
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
            }
        }
    }

    // MARK: - Intro

    private var introText: some View {
        Text("Activer un calendrier de communauté religieuse en plus du calendrier diocésain.")
            .font(.system(size: 15, design: .serif))
            .foregroundStyle(Color.textSecondary)
            .lineSpacing(4)
    }

    // MARK: - Community Picker

    private var communitySection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("COMMUNAUTÉ")
                .cellaSectionHeader()

            VStack(spacing: 0) {
                // "None" option
                SettingsRadioRow(
                    title: "Aucune",
                    isSelected: selectedCommunityId.isEmpty,
                    onSelect: {
                        selectedCommunityId = ""
                        selectedProvinceId = ""
                    },
                    color: liturgicalColor
                )

                ForEach(communities) { community in
                    Divider().padding(.horizontal, CellaTheme.cardPadding)
                    SettingsRadioRow(
                        title: "\(community.name) (\(community.abbreviation))",
                        isSelected: selectedCommunityId == community.id,
                        onSelect: {
                            selectedCommunityId = community.id
                            selectedProvinceId = ""
                        },
                        color: liturgicalColor
                    )
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

    // MARK: - Province Picker

    @ViewBuilder
    private var provinceSection: some View {
        if let community = selectedCommunity, !community.provinces.isEmpty {
            VStack(alignment: .leading, spacing: 12) {
                Text("PROVINCE / MAISON")
                    .cellaSectionHeader()

                VStack(spacing: 0) {
                    SettingsRadioRow(
                        title: "\(community.name) (général)",
                        isSelected: selectedProvinceId.isEmpty,
                        onSelect: { selectedProvinceId = "" },
                        color: liturgicalColor
                    )

                    ForEach(community.provinces) { province in
                        Divider().padding(.horizontal, CellaTheme.cardPadding)
                        SettingsRadioRow(
                            title: province.name,
                            isSelected: selectedProvinceId == province.id,
                            onSelect: { selectedProvinceId = province.id },
                            color: liturgicalColor
                        )
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

    // MARK: - Default Office

    @ViewBuilder
    private var officeDefaultSection: some View {
        if !selectedCommunityId.isEmpty {
            VStack(alignment: .leading, spacing: 12) {
                Text("OFFICE DES HEURES")
                    .cellaSectionHeader()

                Text("La communauté sélectionnée sera proposée comme source dans l'onglet Heures (en plus de l'office romain standard).")
                    .font(CellaFont.caption(13))
                    .foregroundStyle(Color.textTertiary)
                    .lineSpacing(3)

                VStack(alignment: .leading, spacing: 12) {
                    Text("Office par défaut")
                        .font(CellaFont.label(13))
                        .foregroundStyle(Color.textSecondary)

                    VStack(spacing: 0) {
                        SettingsRadioRow(
                            title: "Liturgie des Heures (romain)",
                            isSelected: defaultOffice == 0,
                            onSelect: { defaultOffice = 0 },
                            color: liturgicalColor
                        )
                        Divider().padding(.horizontal, CellaTheme.cardPadding)
                        SettingsRadioRow(
                            title: "Office propre de la communauté",
                            isSelected: defaultOffice == 1,
                            onSelect: { defaultOffice = 1 },
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
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        CommunitySettingsView()
    }
}
