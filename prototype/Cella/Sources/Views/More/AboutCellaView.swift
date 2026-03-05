import SwiftUI

// MARK: - About Cella Screen

struct AboutCellaView: View {
    private let liturgicalColor: Color = MockData.today.color.color

    var body: some View {
        ScrollView {
            VStack(spacing: 0) {
                Rectangle()
                    .fill(liturgicalColor)
                    .frame(height: 2)

                VStack(spacing: 36) {
                    headerSection
                    dividerSymbol
                    monasticCellSection
                    dividerSymbol
                    cellarSection
                    dividerSymbol
                    missionSection
                    dividerSymbol
                    footerSection
                }
                .padding(.horizontal, CellaTheme.horizontalPadding)
                .padding(.vertical, 32)
            }
        }
        .background(Color.parchmentWarm)
        .navigationBarTitleDisplayMode(.inline)
        .toolbar {
            ToolbarItem(placement: .principal) {
                Text("Qu'est-ce que Cella ?")
                    .font(CellaFont.label(15))
                    .foregroundStyle(Color.textPrimary)
            }
        }
    }

    // MARK: - Header

    private var headerSection: some View {
        VStack(spacing: 16) {
            Text("✝")
                .font(.system(size: 28))
                .foregroundStyle(liturgicalColor)

            Text("CELLA")
                .font(.system(size: 32, weight: .light, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .tracking(8)
        }
    }

    // MARK: - Monastic Cell

    private var monasticCellSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("LA CELLULE MONASTIQUE")
                .font(CellaFont.label(12))
                .foregroundStyle(Color.textSecondary)
                .tracking(1.5)

            VStack(alignment: .leading, spacing: 12) {
                Text("Dans la tradition des Pères du désert, la cellule est le lieu du combat spirituel et de la rencontre intime avec Dieu. Abba Moïse disait à celui qui le consultait :")
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)

                Text("« Va, assieds-toi dans ta cellule, et ta cellule t'enseignera tout. »")
                    .font(.system(size: 17, design: .serif).italic())
                    .foregroundStyle(liturgicalColor)
                    .lineSpacing(6)
                    .padding(.vertical, 4)

                Text("La cellule n'est pas une prison ; c'est un espace choisi, librement habité, où le silence permet d'entendre la voix qui parle au cœur.")
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)
            }
        }
    }

    // MARK: - Cellar of the Song

    private var cellarSection: some View {
        VStack(alignment: .leading, spacing: 16) {
            Text("LE CELLIER DU CANTIQUE")
                .font(CellaFont.label(12))
                .foregroundStyle(Color.textSecondary)
                .tracking(1.5)

            VStack(alignment: .leading, spacing: 12) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("« Il m'a introduite dans son cellier,")
                        .font(.system(size: 17, design: .serif).italic())
                        .foregroundStyle(liturgicalColor)

                    Text("et la bannière qu'il dresse sur moi,")
                        .font(.system(size: 17, design: .serif).italic())
                        .foregroundStyle(liturgicalColor)

                    Text("c'est l'amour. »")
                        .font(.system(size: 17, design: .serif).italic())
                        .foregroundStyle(liturgicalColor)

                    Text("— Ct 2,4")
                        .font(CellaFont.caption(13))
                        .foregroundStyle(Color.textTertiary)
                        .padding(.top, 2)
                }
                .padding(.vertical, 4)

                Text("Dans le Cantique des Cantiques, le cellier (cella vinaria) est le lieu de l'ivresse d'amour, où l'Époux — le Christ — fait entrer l'âme dans l'intimité de sa présence.")
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)

                Text("Saint Bernard y voit le sommet de la vie contemplative : être conduit là où le vin de la grâce est en abondance.")
                    .font(.system(size: 16, design: .serif))
                    .foregroundStyle(Color.textPrimary)
                    .lineSpacing(6)
            }
        }
    }

    // MARK: - Mission

    private var missionSection: some View {
        VStack(alignment: .leading, spacing: 12) {
            Text("Cella veut être cet espace sur votre téléphone : un seuil que vous franchissez pour entrer dans la prière. Pas de bruit, pas de distraction. Juste la Parole, les psaumes, et le rythme de la liturgie qui vous porte vers Dieu.")
                .font(.system(size: 16, design: .serif))
                .foregroundStyle(Color.textPrimary)
                .lineSpacing(6)
        }
    }

    // MARK: - Footer

    private var footerSection: some View {
        Text("Propulsé par Romcal, moteur liturgique libre.")
            .font(CellaFont.caption(13))
            .foregroundStyle(Color.textTertiary)
    }

    // MARK: - Divider

    private var dividerSymbol: some View {
        HStack(spacing: 16) {
            Rectangle()
                .fill(Color.separator)
                .frame(height: 0.5)
            Text("·")
                .font(.system(size: 20))
                .foregroundStyle(Color.separator)
            Rectangle()
                .fill(Color.separator)
                .frame(height: 0.5)
        }
    }
}

// MARK: - Preview

#Preview {
    NavigationStack {
        AboutCellaView()
    }
}
