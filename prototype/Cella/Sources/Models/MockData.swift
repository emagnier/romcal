import Foundation

// MARK: - Mock Data

struct MockData {

    // MARK: - Today (Lent Thursday)

    static let today: LiturgicalDay = {
        let cal = Calendar.current
        let date = cal.date(from: DateComponents(year: 2026, month: 3, day: 5))!
        return LiturgicalDay(
            id: "thursday_1st_week_lent",
            date: date,
            title: "Jeudi de la 1re semaine de Carême",
            subtitle: "Férie — Temps du Carême",
            season: .lent,
            color: .violet,
            psalterWeek: 1,
            sundayCycle: "A",
            weekdayCycle: "I",
            celebrations: [
                Celebration(
                    id: "thursday_1st_week_lent",
                    fullname: "Férie du Jeudi — 1re semaine de Carême",
                    rank: .weekday,
                    color: .violet,
                    isOptional: false,
                    martyrologyNote: nil,
                    commons: []
                ),
                Celebration(
                    id: "perpetua_felicity",
                    fullname: "Saintes Perpétue et Félicité, martyres",
                    rank: .optionalMemorial,
                    color: .red,
                    isOptional: true,
                    martyrologyNote: "Martyres à Carthage en 203",
                    commons: ["Commun des martyres"]
                )
            ],
            collect: "Accorde-nous, Seigneur, de trouver notre joie dans ta louange, car c'est en t'aimant sans cesse que nous aurons le bonheur durable ; puisque le bonheur véritable et total est de servir l'auteur de tout bien. Par Jésus Christ, ton Fils, notre Seigneur.",
            prayerOverOfferings: "Que cette offrande, Seigneur, nous purifie et nous renouvelle ; qu'elle soit pour ceux qui font ta volonté la source d'une récompense éternelle. Par Jésus Christ.",
            prayerAfterCommunion: "Dieu de miséricorde, que la communion à ton sacrement nous garde du péché et nous donne la force pour accomplir tes commandements. Par Jésus Christ.",
            martyrologyEntry: MartyrologyEntry(
                id: "perpetua_felicity",
                fullname: "Saintes Perpétue et Félicité",
                shortBio: "Perpétue était une jeune femme noble de Carthage, catéchumène de vingt-deux ans et mère d'un enfant. Félicité, sa servante, était enceinte. Arrêtées avec d'autres catéchumènes en 203, elles furent baptisées en prison puis livrées aux bêtes dans l'amphithéâtre de Carthage, sous Septime Sévère. Le récit de leur passion, en partie rédigé par Perpétue elle-même, est l'un des plus anciens témoignages chrétiens.",
                dateOfDeath: "7 mars 203",
                titles: ["Martyres"],
                canonizationLevel: "Saintes"
            )
        )
    }()

    // MARK: - Another day (Sunday)

    static let sunday: LiturgicalDay = {
        let cal = Calendar.current
        let date = cal.date(from: DateComponents(year: 2026, month: 3, day: 1))!
        return LiturgicalDay(
            id: "1st_sunday_lent",
            date: date,
            title: "1er Dimanche de Carême",
            subtitle: "Dimanche — Temps du Carême",
            season: .lent,
            color: .violet,
            psalterWeek: 4,
            sundayCycle: "A",
            weekdayCycle: "I",
            celebrations: [
                Celebration(
                    id: "1st_sunday_lent",
                    fullname: "1er Dimanche de Carême",
                    rank: .sunday,
                    color: .violet,
                    isOptional: false,
                    martyrologyNote: nil,
                    commons: []
                )
            ],
            collect: "Accorde-nous, Dieu tout-puissant, tout au long de ce Carême, de progresser dans la connaissance de Jésus Christ et de nous ouvrir à sa lumière par une vie de plus en plus fidèle. Lui qui règne.",
            prayerOverOfferings: "Nous t'offrons, Seigneur, de quoi célébrer le sacrifice en ce temps de Carême ; accorde-nous d'y mettre le même esprit de pénitence que dans notre vie quotidienne. Par Jésus Christ.",
            prayerAfterCommunion: "Le pain que nous avons reçu de ta table, Seigneur, doit renouveler le fond de notre cœur ; il rend plus vive en nous la force de ta grâce. Par Jésus Christ.",
            martyrologyEntry: nil
        )
    }()

    // MARK: - Readings

    static let todayFormularies: [MassFormulary] = [
        MassFormulary(
            id: "day_mass",
            name: "Messe du jour",
            readings: [
                MassReading(
                    id: "r1",
                    type: .firstReading,
                    reference: "Est 14, 1.3-5.12-14",
                    text: """
                    En ces jours-là, la reine Esther, saisie d'une angoisse mortelle, \
                    se réfugia auprès du Seigneur.

                    Elle se prosterna contre terre avec ses servantes, du matin jusqu'au soir, et elle dit :

                    « Dieu d'Abraham, Dieu d'Isaac, Dieu de Jacob, tu es béni. \
                    Viens à mon secours, car je suis seule et je n'ai pas d'autre défenseur que toi, Seigneur.

                    Tu connais toutes choses. Tu sais que je hais la gloire des impies \
                    et que j'ai horreur de la couche des incirconcis et de tout étranger.

                    Tu connais la contrainte que je subis. Je déteste l'insigne de ma grandeur, \
                    que je porte sur ma tête aux jours où je me montre en public ; \
                    je le déteste comme un linge souillé et ne le porte pas les jours où je suis tranquille.

                    Souviens-toi, Seigneur ; manifeste-toi au jour de notre détresse. \
                    Et moi, donne-moi du courage, Roi des dieux et Maître de toute autorité.

                    Mets sur mes lèvres une parole harmonieuse quand je serai en présence du lion ; \
                    change son cœur et inspire-lui la haine de celui qui nous combat, \
                    pour sa perte et pour la perte de ceux qui partagent ses desseins.

                    Et nous, délivre-nous par ta main ; \
                    viens me secourir, car je suis seule et je n'ai que toi, Seigneur. »
                    """,
                    response: nil,
                    alternativeIndex: nil,
                    alternativeCount: nil
                ),
                MassReading(
                    id: "ps",
                    type: .psalm,
                    reference: "Ps 137 (138), 1-2a, 2bc-3, 7c-8",
                    text: """
                    De tout mon cœur, Seigneur, je te rends grâce :
                    tu as entendu les paroles de ma bouche.
                    Je te chante en présence des anges,
                    vers ton temple sacré, je me prosterne.

                    Je rends grâce à ton nom pour ton amour et ta vérité,
                    car tu élèves, au-dessus de tout, ton nom et ta parole.
                    Le jour où tu répondis à mon appel,
                    tu fis grandir en mon âme la force.

                    Ta droite me rend vainqueur.
                    Le Seigneur fait tout pour moi !
                    Seigneur, éternel est ton amour :
                    n'arrête pas l'œuvre de tes mains.
                    """,
                    response: "Quand je t'appelle, tu m'exauces, Seigneur.",
                    alternativeIndex: nil,
                    alternativeCount: nil
                ),
                MassReading(
                    id: "acc",
                    type: .acclamation,
                    reference: "Ps 50, 12a.14a",
                    text: """
                    Gloire et louange à toi, Seigneur Jésus.
                    Crée en moi un cœur pur, ô mon Dieu ;
                    rends-moi la joie d'être sauvé.
                    Gloire et louange à toi, Seigneur Jésus.
                    """,
                    response: nil,
                    alternativeIndex: nil,
                    alternativeCount: nil
                ),
                MassReading(
                    id: "ev",
                    type: .gospel,
                    reference: "Mt 7, 7-12",
                    text: """
                    En ce temps-là, Jésus disait à ses disciples :

                    « Demandez, on vous donnera ; cherchez, vous trouverez ; frappez, on vous ouvrira.
                    En effet, quiconque demande reçoit ; qui cherche trouve ; à qui frappe, on ouvrira.

                    Lequel d'entre vous donnerait une pierre à son fils qui lui demande du pain ?
                    ou un serpent, quand il lui demande un poisson ?
                    Si donc vous, qui êtes mauvais, vous savez donner de bonnes choses à vos enfants,
                    combien plus votre Père qui est aux cieux donnera-t-il de bonnes choses
                    à ceux qui les lui demandent !

                    Donc, tout ce que vous voudriez que les autres fassent pour vous,
                    faites-le pour eux, vous aussi :
                    voilà ce que disent la Loi et les Prophètes. »
                    """,
                    response: nil,
                    alternativeIndex: nil,
                    alternativeCount: nil
                )
            ]
        )
    ]

    // MARK: - Commentary

    static let commentary = PatristicCommentary(
        id: "chrysostom_mt7",
        author: "Saint Jean Chrysostome",
        source: "Homélie 23 sur l'Évangile de Matthieu",
        text: """
        Le Seigneur ne dit pas simplement : « Demandez », mais il insiste avec force : \
        « Cherchez. » Car il faut que celui qui cherche Dieu le fasse avec toute la ferveur \
        et l'ardeur dont il est capable, en rejetant loin de lui tout ce qui l'empêche de trouver.

        C'est ce que veulent dire les mots : « Frappez, et l'on vous ouvrira. » Car il faut \
        s'approcher de Dieu avec empressement et une prière ardente. Ne vous découragez pas \
        dans votre demande. Il ne dit pas que le Père vous donnera ceci ou cela, mais \
        « de bonnes choses », laissant entendre que ce qu'on reçoit en demandant, c'est \
        toujours un bien véritable.

        Voyez encore comment il rend la prière aisée : ce n'est pas celui qui siège qui donne, \
        mais celui qui cherche qui trouve ; et ce n'est pas à celui qui attend qu'on ouvre, \
        mais à celui qui frappe. Donc, si tu demandes et ne reçois pas, c'est que tu ne \
        demandes pas encore ; continue de frapper, et tu seras exaucé.
        """,
        sourceType: "Patristique",
        relatedReadingType: .gospel
    )

    // MARK: - Offices

    static let officeSources: [OfficeSource] = [
        OfficeSource(id: "roman", name: "Liturgie des Heures (Église)", communityName: nil),
        OfficeSource(id: "benedictine", name: "Office bénédictin", communityName: "Bénédictins"),
        OfficeSource(id: "dominican", name: "Office dominicain", communityName: "Dominicains"),
        OfficeSource(id: "jerusalem", name: "Fraternités de Jérusalem", communityName: "FMJ")
    ]

    static let laudsContent = OfficeContent(
        id: "lauds_thursday_lent_1",
        type: .lauds,
        sections: [
            OfficeSection(
                title: "Introduction",
                content: """
                ℣ Seigneur, ouvre mes lèvres.
                ℟ Et ma bouche publiera ta louange.
                """
            ),
            OfficeSection(
                title: "Hymne",
                content: """
                Splendeur jaillie du sein de Dieu,
                Lumière née de la lumière,
                Jour, tu dissipes dans les cieux
                La ténébreuse nuit de la terre.

                Vrai soleil, sur nous resplendis
                D'un éclat qui jamais ne décline ;
                Et du Saint-Esprit répandis
                Les rayons dans nos poitrines.

                Invoquons aussi le Père,
                Le Père de la gloire éternelle,
                Le Père de la grâce austère,
                Qu'il écarte tout péché loin d'elle.

                Qu'il donne à nos actes la vigueur,
                Qu'il brise la dent de l'envie,
                Qu'il aide à supporter le malheur,
                Qu'il accorde la grâce et la vie.
                """
            ),
            OfficeSection(
                title: "Psaume 56 (57)",
                subtitle: "Prière du matin dans l'épreuve",
                content: """
                Pitié, mon Dieu, pitié pour moi,
                en toi je me réfugie ;
                je me réfugie à l'ombre de tes ailes
                en attendant que passe le malheur.

                Je crie vers Dieu, le Très-Haut,
                vers Dieu qui fera tout pour moi.
                Du ciel, qu'il m'envoie le salut,
                qu'il confonde celui qui me poursuit ;

                que Dieu m'envoie son amour et sa vérité !
                Je suis couché au milieu de lions,
                des hommes qui ont pour dents des lances et des flèches,
                pour langue, une épée acérée.

                Dieu, lève-toi sur les cieux :
                que ta gloire domine la terre !

                On a tendu un filet sous mes pas :
                j'allais fléchir.
                On a creusé devant moi une fosse :
                c'est eux qui sont tombés.

                Mon cœur est prêt, mon Dieu,
                mon cœur est prêt !
                Je veux chanter, jouer des hymnes !

                Éveille-toi, ma gloire !
                Éveillez-vous, harpe, cithare,
                que j'éveille l'aurore !

                Je te rendrai grâce parmi les peuples, Seigneur,
                et je jouerai pour toi parmi les nations :
                ton amour est grand jusqu'aux cieux,
                ta vérité, jusqu'aux nuages.

                Dieu, lève-toi sur les cieux :
                que ta gloire domine la terre !
                """,
                isAntiphon: false
            ),
            OfficeSection(
                title: "Antienne",
                content: "Pitié, mon Dieu, pitié pour moi : en toi je me réfugie.",
                isAntiphon: true
            ),
            OfficeSection(
                title: "Cantique (1 S 2)",
                subtitle: "Action de grâce de la mère de Samuel",
                content: """
                Mon cœur exulte dans le Seigneur,
                le Seigneur m'a donné la force,
                ma bouche a de quoi confondre mes ennemis :
                oui, je me réjouis de ton salut !

                Nul n'est saint comme le Seigneur,
                il n'y a pas d'autre Dieu que toi ;
                pas de Rocher comme notre Dieu !

                Cessez de parler avec tant d'arrogance,
                que rien d'insolent ne sorte de votre bouche,
                car le Seigneur est le Dieu qui sait tout,
                et par lui les actions sont pesées.
                """
            ),
            OfficeSection(
                title: "Psaume 147 (147B)",
                subtitle: "La Jérusalem restaurée",
                content: """
                Glorifie le Seigneur, Jérusalem !
                Célèbre ton Dieu, ô Sion !

                Il a consolidé les barres de tes portes,
                dans tes murs il a béni tes enfants ;
                il fait régner la paix à tes frontières,
                et d'un pain de froment te rassasie.

                Il envoie sa parole sur la terre :
                rapide, son verbe la parcourt.
                Il étale une toison de neige,
                il sème une poussière de givre.

                Il fait connaître sa parole à Jacob,
                ses volontés et ses lois à Israël.
                Pas un peuple qu'il ait ainsi traité ;
                nul autre n'a connu ses volontés.
                """
            ),
            OfficeSection(
                title: "Lecture brève",
                subtitle: "Is 66, 1-2",
                content: """
                Ainsi parle le Seigneur : Le ciel est mon trône et la terre, l'escabeau de mes pieds. \
                Quelle maison pourriez-vous me bâtir ? En quel lieu me faire reposer ? \
                Tout cela, c'est ma main qui l'a fait, et tout cela est à moi — oracle du Seigneur. \
                Celui que je regarde, c'est le pauvre, l'humilié, celui qui tremble à ma parole.
                """
            ),
            OfficeSection(
                title: "Répons bref",
                content: """
                ℣ Dans la détresse, je crie vers le Seigneur ;
                ℟ Et lui me répond.
                """,
                isResponse: true
            ),
            OfficeSection(
                title: "Cantique de Zacharie",
                subtitle: "Lc 1, 68-79",
                content: """
                Béni soit le Seigneur, le Dieu d'Israël,
                qui visite et rachète son peuple.

                Il a fait surgir la force qui nous sauve
                dans la maison de David, son serviteur,
                comme il l'avait dit par la bouche des saints,
                par ses prophètes, depuis les temps anciens :

                salut qui nous arrache à l'ennemi,
                à la main de tous ceux qui nous haïssent.

                Amour qu'il montre envers nos pères,
                mémoire de son alliance sainte,
                serment juré à notre père Abraham
                de nous rendre sans crainte,

                afin que, délivrés de la main des ennemis,
                nous le servions dans la justice et la sainteté,
                en sa présence, tout au long de nos jours.

                Et toi, petit enfant, tu seras appelé
                prophète du Très-Haut ;
                tu marcheras devant, à la face du Seigneur,
                et tu prépareras ses chemins,

                pour donner à son peuple de connaître le salut
                par la rémission de ses péchés,
                grâce à la tendresse, à l'amour de notre Dieu,
                quand nous visite l'astre d'en haut,

                pour illuminer ceux qui habitent les ténèbres
                et l'ombre de la mort,
                pour conduire nos pas
                au chemin de la paix.
                """
            ),
            OfficeSection(
                title: "Antienne du Benedictus",
                content: "Le Seigneur nous sauve de nos ennemis et de la main de tous ceux qui nous haïssent.",
                isAntiphon: true
            ),
            OfficeSection(
                title: "Intercessions",
                content: """
                Prions le Christ, le soleil de justice qui éclaire tout homme :

                ℟ Illumine-nous, Seigneur !

                Béni sois-tu, Créateur de la lumière : tu nous donnes la lumière de ce jour nouveau.
                ℟ Illumine-nous, Seigneur !

                Tu es apparu aux apôtres après ta résurrection : brille dans nos cœurs aujourd'hui.
                ℟ Illumine-nous, Seigneur !

                Tu nous invites à marcher dans la lumière : que nos œuvres en ce jour te soient agréables.
                ℟ Illumine-nous, Seigneur !

                Éclaire nos yeux pour que jamais ils ne dorment dans la mort,
                ℟ Illumine-nous, Seigneur !
                """,
                isResponse: true
            ),
            OfficeSection(
                title: "Notre Père",
                content: """
                Notre Père, qui es aux cieux,
                que ton nom soit sanctifié,
                que ton règne vienne,
                que ta volonté soit faite sur la terre comme au ciel.
                Donne-nous aujourd'hui notre pain de ce jour.
                Pardonne-nous nos offenses,
                comme nous pardonnons aussi à ceux qui nous ont offensés.
                Et ne nous laisse pas entrer en tentation
                mais délivre-nous du Mal.
                """
            ),
            OfficeSection(
                title: "Oraison",
                content: """
                Accorde-nous, Seigneur, de trouver notre joie dans ta louange, \
                car c'est en t'aimant sans cesse que nous aurons le bonheur durable ; \
                puisque le bonheur véritable et total est de servir l'auteur de tout bien. \
                Par Jésus Christ, ton Fils, notre Seigneur.
                """
            ),
            OfficeSection(
                title: "Conclusion",
                content: """
                ℣ Que le Seigneur nous bénisse, qu'il nous garde de tout mal et nous conduise à la vie éternelle.
                ℟ Amen.
                """
            )
        ]
    )

    // MARK: - Calendar Settings Data

    static let regions: [CalendarRegion] = [
        CalendarRegion(id: "europe", name: "Europe", countries: [
            CalendarCountry(id: "france", name: "France", dioceses: [
                CalendarDiocese(id: "france__paris", name: "Paris"),
                CalendarDiocese(id: "france__lyon", name: "Lyon"),
                CalendarDiocese(id: "france__strasbourg", name: "Strasbourg"),
                CalendarDiocese(id: "france__marseille", name: "Marseille"),
                CalendarDiocese(id: "france__toulouse", name: "Toulouse")
            ]),
            CalendarCountry(id: "italy", name: "Italie", dioceses: [
                CalendarDiocese(id: "italy__roma", name: "Rome"),
                CalendarDiocese(id: "italy__milano", name: "Milan")
            ]),
            CalendarCountry(id: "spain", name: "Espagne", dioceses: []),
            CalendarCountry(id: "germany", name: "Allemagne", dioceses: []),
            CalendarCountry(id: "belgium", name: "Belgique", dioceses: [])
        ]),
        CalendarRegion(id: "americas", name: "Amériques", countries: [
            CalendarCountry(id: "united_states", name: "États-Unis", dioceses: []),
            CalendarCountry(id: "canada", name: "Canada", dioceses: []),
            CalendarCountry(id: "brazil", name: "Brésil", dioceses: [])
        ]),
        CalendarRegion(id: "africa", name: "Afrique", countries: []),
        CalendarRegion(id: "asia", name: "Asie", countries: []),
        CalendarRegion(id: "oceania", name: "Océanie", countries: [])
    ]

    static let communities: [ReligiousCommunity] = [
        ReligiousCommunity(id: "benedictines", name: "Bénédictins", abbreviation: "OSB", provinces: [
            CommunityProvince(id: "benedictines__france", name: "France"),
            CommunityProvince(id: "benedictines__france__solesmes", name: "Solesmes")
        ]),
        ReligiousCommunity(id: "dominicans", name: "Dominicains", abbreviation: "OP", provinces: [
            CommunityProvince(id: "dominicans__france", name: "Province de France")
        ]),
        ReligiousCommunity(id: "franciscans", name: "Franciscains", abbreviation: "OFM", provinces: []),
        ReligiousCommunity(id: "jerusalem", name: "Fraternités Monastiques de Jérusalem", abbreviation: "FMJ", provinces: []),
        ReligiousCommunity(id: "carmelites", name: "Carmélites", abbreviation: "OCD", provinces: []),
        ReligiousCommunity(id: "jesuits", name: "Jésuites", abbreviation: "SJ", provinces: [])
    ]
}
