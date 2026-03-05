# Cella — Design de l'application iOS

> *« Il m'a introduite dans son cellier, et la bannière qu'il dresse sur moi, c'est l'amour. »*
> — Cantique des Cantiques 2,4

## 1. Vision et identité

### 1.1 Le nom : Cella

**Cella** — du latin *cella* — porte une double résonance :

- **La cellule monastique** : un espace de retrait, de silence et de rencontre intime avec Dieu. La tradition monastique enseigne : *« Reste dans ta cellule, et ta cellule t'apprendra tout. »* (Abba Moïse)
- **Le cellier du Cantique des Cantiques** (*cella vinaria*) : lieu où l'Époux introduit l'âme aimée, lieu d'ivresse spirituelle et d'union. Un espace de surabondance et de joie intérieure.

L'app Cella aspire à devenir ce lieu numérique — paradoxalement dépouillé et riche — où l'utilisateur entre dans le silence de sa cellule pour y trouver la Parole, la prière et la présence.

### 1.2 Principes directeurs du design

| Principe | Traduction concrète |
|---|---|
| **Silence visuel** | Pas de notifications intrusives, pas d'animations superflues. Chaque pixel sert la prière. |
| **Sobriété monastique** | Palette restreinte, typographie noble et lisible, espaces généreux. L'écran respire. |
| **Profondeur sans complexité** | Toute la richesse liturgique de Romcal est accessible, mais révélée progressivement — jamais imposée. |
| **Rythme liturgique** | L'app s'adapte au temps : couleur liturgique du jour, office suggéré selon l'heure, saison visuelle. |
| **Aucune distraction** | Zéro publicité, zéro tracking, zéro gamification. L'app est un outil au service de la prière, pas un produit d'attention. |

---

## 2. Identité visuelle

### 2.1 Palette de couleurs

La palette principale est sobre et chaleureuse, évoquant la pierre, le parchemin et la lumière tamisée d'un cloître.

```
Fond principal :       #FAF7F2  (parchemin clair, presque blanc)
Fond secondaire :      #F0EBE3  (parchemin chaud)
Fond sombre (dark) :   #1C1A17  (noir chaud, encre de moine)
Fond sombre secondaire:#2A2722  (anthracite chaud)

Texte principal :      #2C2520  (brun très foncé, proche du noir)
Texte secondaire :     #7A7067  (gris chaud)
Texte tertiaire :      #A89F95  (gris clair chaud)

Accent principal :     #8B1A1A  (rouge sombre, évoquant les lettrines)
Accent doux :          #C4956A  (or patiné, pour les accents discrets)
Séparateurs :          #E8E2DA  (ligne fine, presque invisible)
```

**Couleurs liturgiques** — utilisées comme accents subtils (fine barre latérale, pastille, filet) :

```
Blanc/Doré :   #C4956A  (or patiné)
Rouge :        #8B1A1A  (rouge sombre)
Vert :         #4A6741  (vert sauge)
Violet :       #5B3A6B  (violet profond)
Rose :         #B07A8A  (rose ancien)
Noir :         #2C2520  (brun noir)
```

### 2.2 Typographie

| Usage | Police | Caractéristiques |
|---|---|---|
| Titres liturgiques, noms des fêtes | **Cormorant Garamond** (serif) | Élégante, classique, rappelle les livres liturgiques |
| Corps de texte, lectures | **Source Serif 4** (serif) | Très lisible en lecture longue, tons humanistes |
| Rubriques, labels, UI | **Inter** (sans-serif) | Claire, discrète, excellente lisibilité petite taille |
| Textes latins, antiennes | **Cormorant Garamond Italic** | Distingue le latin du vernaculaire |

### 2.3 Iconographie

Style : **traits fins, linéaires**, inspirés des gravures médiévales et de l'art roman.
Pas d'icônes remplies/opaques. Tout est aérien, minimal, sacré.

Icônes de la tab bar :
- **Aujourd'hui** : cercle solaire avec croix (jour liturgique)
- **Lectures** : livre ouvert
- **Heures** : cloche (rappelant la cloche monastique qui rythme les offices)
- **Plus** : croix simple

---

## 3. Architecture de navigation

### 3.1 Tab Bar principale (4 onglets)

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│                   [Contenu de l'onglet]                  │
│                                                         │
├────────────┬────────────┬────────────┬──────────────────┤
│  ☉         │  📖        │  🔔        │  ✚              │
│ Aujourd'hui│  Lectures  │  Heures    │  Plus            │
└────────────┴────────────┴────────────┴──────────────────┘
```

---

## 4. Onglet 1 — Aujourd'hui

### 4.1 Vue d'ensemble

L'onglet « Aujourd'hui » est la porte d'entrée de la prière quotidienne. Il donne en un regard l'identité liturgique du jour et propose un accès direct à l'office approprié selon l'heure.

### 4.2 Structure de l'écran (scroll vertical)

```
┌─────────────────────────────────────────────────┐
│  ← 4 mars                                  5 mars →  │  Swipe horizontal
│                                                       │  pour naviguer
│           Jeudi de la 1re semaine de Carême            │  entre les jours
│                                                       │
│  ▌violet   Férie — Temps du Carême                    │  Barre de couleur
│            Semaine du Psautier : I                     │  liturgique
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  OFFICE SUGGÉRÉ                              14:32    │
│  ┌───────────────────────────────────────────────┐    │
│  │  🔔  None — Prière du milieu du jour          │    │  Carte mise en
│  │      « Seigneur, apprends-nous à compter      │    │  avant selon
│  │        nos jours… »                           │    │  l'heure actuelle
│  │                                    Prier →    │    │
│  └───────────────────────────────────────────────┘    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  CÉLÉBRATIONS DU JOUR                                 │
│                                                       │
│  ┌───────────────────────────────────────────────┐    │
│  │  ◉ Férie du Jeudi — 1re sem. de Carême        │    │  Sélection
│  │     Messe du jour                             │    │  principale
│  └───────────────────────────────────────────────┘    │
│  ┌───────────────────────────────────────────────┐    │
│  │  ○ Ste Perpétue et Ste Félicité, martyres     │    │  Mémoire
│  │     Mémoire facultative                ⓘ     │    │  facultative
│  └───────────────────────────────────────────────┘    │
│                                                       │
│  Le bouton radio ◉/○ permet de basculer entre         │
│  les célébrations et adapte tout l'écran               │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ORAISON DU JOUR                                      │
│                                                       │
│  Collecte                                             │
│  « Accorde-nous, Seigneur, de trouver notre joie      │
│    dans ta louange, car c'est en t'aimant sans         │
│    cesse que nous aurons le bonheur durable… »         │
│                                                       │
│  ┌──────────────┐  ┌──────────────────────┐           │
│  │ Sur les       │  │ Après la              │           │
│  │ offrandes     │  │ communion             │           │
│  └──────────────┘  └──────────────────────┘           │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  MÉMOIRE DU JOUR                                      │
│                                                       │
│  ┌───────────────────────────────────────────────┐    │
│  │  Saintes Perpétue et Félicité                  │    │
│  │  Martyres à Carthage (✝ 203)                   │    │
│  │                                                │    │
│  │  « Elles furent livrées aux bêtes dans         │    │
│  │    l'amphithéâtre de Carthage… »                │    │
│  │                                  Lire plus →  │    │
│  └───────────────────────────────────────────────┘    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  TOUS LES OFFICES                                     │
│                                                       │
│  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐         │
│  │ Vigiles │ │ Laudes │ │ Tierce │ │ Sexte  │         │
│  │  ✓     │ │  ✓     │ │        │ │        │         │
│  └────────┘ └────────┘ └────────┘ └────────┘         │
│  ┌────────┐ ┌────────┐ ┌────────┐                     │
│  │ None   │ │Vêpres  │ │Complies│                     │
│  │  ●     │ │        │ │        │                     │
│  └────────┘ └────────┘ └────────┘                     │
│                                                       │
│  ✓ = prié   ● = en cours / suggéré                    │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 4.3 Fonctionnalités clés

#### Sélection de la célébration du jour

Quand Romcal propose plusieurs célébrations (mémoires facultatives, choix entre férie et mémoire, etc.), une section **« Célébrations du jour »** apparaît avec :

- **Un bouton radio** (`◉` / `○`) devant chaque célébration possible
- La célébration de rang le plus élevé est sélectionnée par défaut
- **Basculer la sélection** adapte dynamiquement :
  - L'oraison affichée (collecte, prière sur les offrandes, postcommunion)
  - Les lectures si elles diffèrent (propres vs communes)
  - La couleur liturgique de l'accent
  - La mémoire du jour (notice du martyrologe)

Cette mécanique est le reflet direct des données de Romcal :
- `LiturgicalDay.is_optional` et `Rank::OptionalMemorial` signalent les alternatives
- `MassContext.optional_celebrations: Vec<CelebrationSummary>` fournit la liste
- Pendant le Carême, les mémoires obligatoires deviennent facultatives (GNLY §14) — l'UI le reflète automatiquement

#### Badge d'information `ⓘ`

Un petit badge discret `ⓘ` sur les célébrations facultatives ouvre un **bottom sheet** expliquant :
- Le rang liturgique (Mémoire facultative, Férie, etc.)
- Pourquoi ce choix est possible (ex: « Pendant le Carême, les mémoires obligatoires deviennent facultatives »)
- La source normative (GNLY §14, CP §25, etc.)

#### Office suggéré selon l'heure

L'app propose automatiquement l'office le plus pertinent selon l'heure :

| Tranche horaire | Office suggéré |
|---|---|
| 00:00 – 05:59 | Office des lectures (Vigiles) |
| 06:00 – 08:59 | Laudes |
| 09:00 – 11:29 | Tierce |
| 11:30 – 13:59 | Sexte |
| 14:00 – 16:59 | None |
| 17:00 – 20:29 | Vêpres |
| 20:30 – 23:59 | Complies |

La carte « Office suggéré » affiche :
- Le nom de l'office
- Une antienne ou un verset d'ouverture
- Un bouton « Prier → » qui ouvre directement l'office dans l'onglet Heures

#### Navigation entre les jours

- **Swipe horizontal** pour passer au jour précédent/suivant
- **Tap sur la date** pour ouvrir un calendrier mensuel (vue compacte)
- **Aujourd'hui** : bouton de retour rapide au jour courant (apparaît quand on navigue)

---

## 5. Onglet 2 — Lectures

### 5.1 Vue d'ensemble

L'onglet « Lectures » présente les lectures de la messe du jour dans un cadre propice à la *lectio divina*, avec un commentaire spirituel ou patristique en lien avec l'une des lectures.

### 5.2 Structure de l'écran

```
┌───────────────────────────────────────────────────────┐
│                                                       │
│  JEUDI DE LA 1RE SEMAINE DE CARÊME                    │
│  Année A — Cycle I                                    │
│                                                       │
│  ┌─ Choix du formulaire ─────────────────────────┐    │
│  │  Messe du jour                            ▼   │    │  Menu déroulant
│  └───────────────────────────────────────────────┘    │  si plusieurs
│                                                       │  formulaires
├───────────────────────────────────────────────────────┤
│                                                       │
│  ── Première lecture ──────────────────────────────    │
│                                                       │
│  Livre d'Esther 14, 1.3-5.12-14                       │
│                                                       │
│  « En ces jours-là, la reine Esther, saisie           │
│    d'une angoisse mortelle, se réfugia auprès          │
│    du Seigneur. Elle se prosterna contre terre          │
│    avec ses servantes, du matin jusqu'au soir,          │
│    et elle dit : … »                                   │
│                                                       │
│  — Parole du Seigneur.                                │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ── Psaume responsorial ──────────────────────────    │
│                                                       │
│  Ps 137 (138), 1-2a, 2bc-3, 7c-8                     │
│                                                       │
│  ℟  Quand je t'appelle, tu m'exauces, Seigneur.      │
│                                                       │
│  Je te rends grâce de tout mon cœur,                  │
│  tu as entendu les paroles de ma bouche.               │
│  Je te chante en présence des anges,                   │
│  vers ton temple sacré, je me prosterne. ℟             │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ── Acclamation de l'Évangile ────────────────────    │
│                                                       │
│  Gloire et louange à toi, Seigneur Jésus.             │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ── Évangile ─────────────────────────────────────    │
│                                                       │
│  ✛ Évangile de Jésus Christ selon saint Matthieu      │
│  Mt 7, 7-12                                           │
│                                                       │
│  « En ce temps-là, Jésus disait à ses disciples :     │
│    "Demandez, on vous donnera ; cherchez, vous         │
│    trouverez ; frappez, on vous ouvrira.               │
│    …" »                                               │
│                                                       │
│  — Acclamons la Parole de Dieu.                       │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ── Commentaire ──────────────────────────────────    │
│                                                       │
│  ┌───────────────────────────────────────────────┐    │
│  │  ✝ Saint Jean Chrysostome                      │    │
│  │  Homélie 23 sur l'Évangile de Matthieu         │    │
│  │                                                │    │
│  │  « Le Seigneur ne dit pas simplement :          │    │
│  │    "Demandez", mais il insiste avec force :      │    │
│  │    "Cherchez." Car il faut que celui qui          │    │
│  │    cherche Dieu le fasse avec toute la           │    │
│  │    ferveur et l'ardeur dont il est capable,      │    │
│  │    en rejetant loin de lui tout ce qui           │    │
│  │    l'empêche de trouver… »                      │    │
│  │                                                │    │
│  │  Source : Patristique                     ⓘ   │    │
│  └───────────────────────────────────────────────┘    │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 5.3 Fonctionnalités clés

#### Choix du formulaire de messe

Quand plusieurs formulaires sont disponibles (correspondant aux `MassTime` de Romcal), un **sélecteur discret** apparaît en haut :

```
┌─────────────────────────────────────────┐
│  Messe du jour                      ▼  │
├─────────────────────────────────────────┤
│  ○ Messe de la vigile                   │
│  ◉ Messe du jour                        │
│  ○ Messe à l'aurore                     │
└─────────────────────────────────────────┘
```

Cas typiques :
- **Noël** : Messe de la nuit, de l'aurore, du jour
- **Vigile pascale** : Formulaire spécifique avec 7+ lectures
- **Fêtes avec vigile** : Messe de la veille au soir, messe du jour

#### Choix des lectures alternatives

Quand Romcal fournit plusieurs options de lectures pour un même formulaire (via `MassCycleDefinition` avec des cycles `YearA`/`YearB`/`YearC` ou `Year1`/`Year2`), un **indicateur discret** apparaît :

```
── Première lecture ─────────────── 1/2 ──
                                    ▲ ▼
```

- Les flèches `▲ ▼` (ou un swipe vertical sur la lecture) permettent de parcourir les alternatives
- Un badge `1/2` indique la position dans les options
- Le choix par défaut correspond au cycle liturgique en cours (Année A, Cycle I, etc.)

#### Choix des lectures selon la célébration sélectionnée

Si l'utilisateur a choisi une mémoire facultative dans l'onglet 1, les lectures s'adaptent :
- **Lectures propres** de la mémoire si elles existent
- **Lectures du commun** correspondant (via le `Common` enum de Romcal) sinon
- **Lectures de la férie** toujours accessibles en alternative

Un bandeau discret indique la source :

```
┌───────────────────────────────────────────────┐
│  📖 Lectures propres de Ste Perpétue et       │
│     Ste Félicité                               │
│                                                │
│  Voir aussi : Lectures de la férie →          │
└───────────────────────────────────────────────┘
```

#### Section Commentaire

Sous les lectures, un commentaire spirituel ou patristique :
- **Source** : Pères de l'Église, Docteurs, Saints, auteurs spirituels contemporains
- **Lien** : Toujours en rapport avec l'une des lectures du jour (indiqué par une référence)
- **Longueur** : Court (3-5 paragraphes), adapté à la *lectio divina*
- **Badge source** : `Patristique`, `Spirituel`, `Magistère`, etc.

---

## 6. Onglet 3 — Heures (Liturgie des Heures)

### 6.1 Vue d'ensemble

L'onglet « Heures » donne accès complet à la Liturgie des Heures du jour, structurée selon les sept offices traditionnels, avec support des offices propres de communautés religieuses.

### 6.2 Structure — Écran principal (sélection de l'office)

```
┌───────────────────────────────────────────────────────┐
│                                                       │
│  LITURGIE DES HEURES                                  │
│  Jeudi — 1re semaine de Carême                        │
│  Psautier : Semaine I                                 │
│                                                       │
│  ┌─ Source ──────────────────────────────────────┐    │
│  │  Liturgie des Heures (Église)            ▼   │    │
│  └───────────────────────────────────────────────┘    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│   ┌───────────────────────────────────────────┐       │
│   │                                           │       │
│   │  🌙  OFFICE DES LECTURES                  │       │
│   │      Vigiles                              │       │
│   │      Hymne · Psaumes · Lectures           │       │
│   │                                           │       │
│   └───────────────────────────────────────────┘       │
│                                                       │
│   ┌───────────────────────────────────────────┐       │
│   │                                           │       │
│   │  🌅  LAUDES                               │  ●    │  ● = suggéré
│   │      Prière du matin                      │       │
│   │      Hymne · Psaumes · Cantique ·         │       │
│   │      Intercessions                        │       │
│   │                                           │       │
│   └───────────────────────────────────────────┘       │
│                                                       │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐           │
│   │  TIERCE  │  │  SEXTE   │  │  NONE    │           │
│   │  9h      │  │  12h     │  │  15h     │           │
│   └──────────┘  └──────────┘  └──────────┘           │
│      Prière du milieu du jour                         │
│                                                       │
│   ┌───────────────────────────────────────────┐       │
│   │                                           │       │
│   │  🌇  VÊPRES                               │       │
│   │      Prière du soir                       │       │
│   │      Hymne · Psaumes · Cantique ·         │       │
│   │      Intercessions                        │       │
│   │                                           │       │
│   └───────────────────────────────────────────┘       │
│                                                       │
│   ┌───────────────────────────────────────────┐       │
│   │                                           │       │
│   │  🌙  COMPLIES                              │       │
│   │      Prière de la nuit                    │       │
│   │      Hymne · Psaume · Cantique de Siméon  │       │
│   │                                           │       │
│   └───────────────────────────────────────────┘       │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 6.3 Structure — Vue d'un office (ex: Laudes)

```
┌───────────────────────────────────────────────────────┐
│  ←  LAUDES — Prière du matin                          │
│      Jeudi, 1re semaine de Carême                     │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  INTRODUCTION                                         │
│                                                       │
│  ℣ Seigneur, ouvre mes lèvres.                        │
│  ℟ Et ma bouche publiera ta louange.                  │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  HYMNE                                                │
│                                                       │
│  Splendeur jaillie du sein de Dieu,                   │
│  Lumière née de la lumière,                            │
│  Jour, tu dissipes dans les cieux                      │
│  La ténébreuse nuit de la terre.                       │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  PSAUME 56 (57)                                       │
│  Prière du matin dans l'épreuve                       │
│                                                       │
│  Ant. Pitié, mon Dieu, pitié pour moi :               │
│       en toi je me réfugie.                            │
│                                                       │
│  Pitié, mon Dieu, pitié pour moi,                     │
│  en toi je me réfugie ;                                │
│  je me réfugie à l'ombre de tes ailes                  │
│  en attendant que passe le malheur.                    │
│  …                                                    │
│                                                       │
│  Gloire au Père, et au Fils,                           │
│  et au Saint-Esprit…                                   │
│                                                       │
│  Ant. Pitié, mon Dieu, pitié pour moi :               │
│       en toi je me réfugie.                            │
│                                                       │
│  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄    │
│                                                       │
│  CANTIQUE (Is 48)                                     │
│  …                                                    │
│                                                       │
│  ┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄    │
│                                                       │
│  PSAUME 147 (147B)                                    │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  LECTURE BRÈVE                                        │
│  Is 66, 1-2                                           │
│                                                       │
│  « Ainsi parle le Seigneur : Le ciel est mon           │
│    trône et la terre, l'escabeau de mes pieds… »       │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  RÉPONS BREF                                          │
│  ℣ …                                                  │
│  ℟ …                                                  │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  CANTIQUE DE ZACHARIE (Lc 1, 68-79)                   │
│                                                       │
│  Ant. Le Seigneur nous sauve                           │
│       de nos ennemis.                                  │
│                                                       │
│  Béni soit le Seigneur, le Dieu d'Israël,             │
│  qui visite et rachète son peuple.                     │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  INTERCESSIONS                                        │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  NOTRE PÈRE                                           │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ORAISON                                              │
│  …                                                    │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  ℣ Que le Seigneur nous bénisse,                       │
│    qu'il nous garde de tout mal                        │
│    et nous conduise à la vie éternelle.                │
│  ℟ Amen.                                              │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 6.4 Fonctionnalités clés

#### Sélection de la source liturgique

Le sélecteur **« Source »** en haut de l'écran permet de choisir entre :

```
┌─────────────────────────────────────────┐
│  Source de l'office                  ▼  │
├─────────────────────────────────────────┤
│  ◉ Liturgie des Heures (Église)         │  Défaut
│  ○ Office bénédictin                    │
│  ○ Office dominicain                    │
│  ○ Fraternités de Jérusalem             │
│  ○ …                                   │
└─────────────────────────────────────────┘
```

Ce sélecteur reflète la double hiérarchie de Romcal :
- **Hiérarchie territoriale** : `general_roman → france → france__paris`
- **Hiérarchie religieuse** : `general_roman → benedictines → benedictines__france__solesmes`

Quand un office propre communautaire est sélectionné, les éléments spécifiques (hymnes, antiennes, lectures) sont substitués selon les règles de composition de GILH §225-240, en utilisant le Layer 2 Hours de Romcal.

#### Adaptation au rang liturgique

L'office s'adapte automatiquement au rang (depuis `Rank` et `Precedence`) :
- **Solennité** : Office complet avec Te Deum, premières Vêpres la veille
- **Fête** : Te Deum, pas de premières Vêpres
- **Mémoire** : Overlay — antienne propre remplace l'antienne férial
- **Mémoire facultative** : Overlay optionnel (l'utilisateur choisit)
- **Férie** : Office du psalter de la semaine

#### Cycle du psautier

Le cycle de 4 semaines est automatiquement calculé par Romcal (`psalter_week`):
- Affiché en haut de l'écran : « Psautier : Semaine I »
- Règle spéciale du Carême : la 1re semaine commence au cycle IV (GILH §133)

#### Mode « prière guidée »

Un bouton optionnel **« Prier pas à pas »** transforme l'office en un mode guidé :
- Chaque section apparaît une à une (tap pour avancer)
- Les indications (℣ / ℟, debout / assis) sont mises en avant
- L'écran reste allumé pendant la prière
- Discret, optionnel, pour ceux qui découvrent la Liturgie des Heures

---

## 7. Onglet 4 — Plus

### 7.1 Vue d'ensemble

L'onglet « Plus » est un espace d'accueil pour les fonctionnalités complémentaires et futures, ainsi qu'un accès aux réglages de l'app.

### 7.2 Structure de l'écran

```
┌───────────────────────────────────────────────────────┐
│                                                       │
│  CELLA                                                │
│                                                       │
│  « Il m'a introduite dans son cellier… »              │
│  Ct 2,4                                               │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  EXPLORER                                             │
│                                                       │
│  ┌─────────────────────────────────────────────┐      │
│  │  📖  Bible                                   │      │
│  │      Textes de l'Écriture Sainte             │      │
│  │                                    Bientôt  │      │
│  ├─────────────────────────────────────────────┤      │
│  │  🙏  Recueil de prières                      │      │
│  │      Prières traditionnelles et dévotions    │      │
│  │                                    Bientôt  │      │
│  ├─────────────────────────────────────────────┤      │
│  │  ⛪  Messe.info                              │      │
│  │      Horaires des messes près de chez vous   │      │
│  │                                    Bientôt  │      │
│  ├─────────────────────────────────────────────┤      │
│  │  🎧  Podcasts                                │      │
│  │      Audio d'une communauté ou paroisse      │      │
│  │                                    Bientôt  │      │
│  ├─────────────────────────────────────────────┤      │
│  │  📅  Agenda paroissial                       │      │
│  │      Événements de votre communauté          │      │
│  │                                    Bientôt  │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  RÉGLAGES                                             │
│                                                       │
│  ┌─────────────────────────────────────────────┐      │
│  │  ⚙  Calendrier & Lieu                        │  →  │
│  ├─────────────────────────────────────────────┤      │
│  │  🏠  Communauté religieuse                   │  →  │
│  ├─────────────────────────────────────────────┤      │
│  │  🌓  Apparence                               │  →  │
│  ├─────────────────────────────────────────────┤      │
│  │  🔤  Langue                                  │  →  │
│  └─────────────────────────────────────────────┘      │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  À PROPOS                                             │
│                                                       │
│  ┌─────────────────────────────────────────────┐      │
│  │  ✝  Qu'est-ce que Cella ?                    │  →  │
│  ├─────────────────────────────────────────────┤      │
│  │      Propulsé par Romcal                     │  →  │
│  └─────────────────────────────────────────────┘      │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 7.3 Écran « Qu'est-ce que Cella ? »

```
┌───────────────────────────────────────────────────────┐
│  ←  Qu'est-ce que Cella ?                             │
│                                                       │
│                    ✝                                  │
│                                                       │
│                  CELLA                                 │
│                                                       │
│  ─────────────────────────────────────────────────    │
│                                                       │
│  Cella est un mot latin qui signifie à la fois :      │
│                                                       │
│  LA CELLULE MONASTIQUE                                │
│                                                       │
│  Dans la tradition des Pères du désert, la cellule    │
│  est le lieu du combat spirituel et de la rencontre   │
│  intime avec Dieu. Abba Moïse disait à celui qui      │
│  le consultait : « Va, assieds-toi dans ta cellule,   │
│  et ta cellule t'enseignera tout. »                    │
│                                                       │
│  La cellule n'est pas une prison ; c'est un espace    │
│  choisi, librement habité, où le silence permet        │
│  d'entendre la voix qui parle au cœur.                 │
│                                                       │
│  ─────────────────────────────────────────────────    │
│                                                       │
│  LE CELLIER DU CANTIQUE                               │
│                                                       │
│  « Il m'a introduite dans son cellier,                │
│    et la bannière qu'il dresse sur moi,                │
│    c'est l'amour. »  — Ct 2,4                         │
│                                                       │
│  Dans le Cantique des Cantiques, le cellier            │
│  (cella vinaria) est le lieu de l'ivresse              │
│  d'amour, où l'Époux — le Christ — fait entrer        │
│  l'âme dans l'intimité de sa présence.                 │
│  Saint Bernard y voit le sommet de la vie              │
│  contemplative : être conduit là où le vin de          │
│  la grâce est en abondance.                            │
│                                                       │
│  ─────────────────────────────────────────────────    │
│                                                       │
│  Cella veut être cet espace sur votre téléphone :      │
│  un seuil que vous franchissez pour entrer dans        │
│  la prière. Pas de bruit, pas de distraction.          │
│  Juste la Parole, les psaumes, et le rythme            │
│  de la liturgie qui vous porte vers Dieu.              │
│                                                       │
│  ─────────────────────────────────────────────────    │
│                                                       │
│  Propulsé par Romcal, moteur liturgique libre.         │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 7.4 Écran « Calendrier & Lieu »

Cet écran expose toute la puissance de configuration de Romcal, traduite en UI intuitive.

```
┌───────────────────────────────────────────────────────┐
│  ←  Calendrier & Lieu                                 │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  CALENDRIER LITURGIQUE                                │
│                                                       │
│  Région                                               │
│  ┌─────────────────────────────────────────────┐      │
│  │  Europe                                  ▼  │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
│  Pays                                                 │
│  ┌─────────────────────────────────────────────┐      │
│  │  France                                  ▼  │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
│  Diocèse (optionnel)                                  │
│  ┌─────────────────────────────────────────────┐      │
│  │  Paris                                   ▼  │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
│  La sélection suit la hiérarchie de calendriers       │
│  de Romcal : general_roman → europe → france →        │
│  france__paris                                        │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  OPTIONS RÉGIONALES                                   │
│                                                       │
│  Épiphanie le dimanche               ┌────┐           │
│  (entre le 2 et le 8 janvier)        │ ON │           │
│                                      └────┘           │
│  Ascension le dimanche               ┌─────┐          │
│  (7e dimanche de Pâques)             │ OFF │          │
│                                      └─────┘          │
│  Fête-Dieu le dimanche               ┌────┐           │
│  (dimanche après la Trinité)         │ ON │           │
│                                      └────┘           │
│                                                       │
│  Ces options correspondent aux champs                  │
│  epiphany_on_sunday, ascension_on_sunday,             │
│  corpus_christi_on_sunday du Preset Romcal.           │
│  Les valeurs par défaut proviennent du                │
│  ParticularConfig du calendrier sélectionné.          │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  CALCUL DE PÂQUES                                     │
│                                                       │
│  ┌─────────────────────────────────────────────┐      │
│  │  ◉ Grégorien (1583+)                        │      │
│  │  ○ Julien (326+)                             │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  CADRAGE DE L'ANNÉE                                   │
│                                                       │
│  ┌─────────────────────────────────────────────┐      │
│  │  ◉ Civil (janvier → décembre)                │      │
│  │  ○ Liturgique (Avent → Christ Roi)           │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
└───────────────────────────────────────────────────────┘
```

### 7.5 Écran « Communauté religieuse »

```
┌───────────────────────────────────────────────────────┐
│  ←  Communauté religieuse                             │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  Activer un calendrier de communauté                  │
│  religieuse en plus du calendrier diocésain.          │
│                                                       │
│  COMMUNAUTÉ                                           │
│  ┌─────────────────────────────────────────────┐      │
│  │  Aucune                                  ▼  │      │
│  ├─────────────────────────────────────────────┤      │
│  │  ○ Aucune                                   │      │
│  │  ○ Bénédictins (OSB)                        │      │
│  │  ○ Dominicains (OP)                          │      │
│  │  ○ Franciscains (OFM)                        │      │
│  │  ○ Frat. Monastiques de Jérusalem            │      │
│  │  ○ Carmélites (OCD)                          │      │
│  │  ○ Jésuites (SJ)                             │      │
│  │  ○ …                                        │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
│  PROVINCE / MAISON (si applicable)                    │
│  ┌─────────────────────────────────────────────┐      │
│  │  Sélectionner…                           ▼  │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
│  Apparaît uniquement quand la communauté a des        │
│  sous-calendriers (ex: benedictines →                 │
│  benedictines__france → benedictines__solesmes)       │
│                                                       │
├───────────────────────────────────────────────────────┤
│                                                       │
│  OFFICE DES HEURES                                    │
│                                                       │
│  La communauté sélectionnée sera proposée comme       │
│  source dans l'onglet Heures (en plus de l'office     │
│  romain standard).                                    │
│                                                       │
│  Office par défaut                                    │
│  ┌─────────────────────────────────────────────┐      │
│  │  ◉ Liturgie des Heures (romain)              │      │
│  │  ○ Office propre de la communauté            │      │
│  └─────────────────────────────────────────────┘      │
│                                                       │
└───────────────────────────────────────────────────────┘
```

---

## 8. Comportements transversaux

### 8.1 Adaptation à la couleur liturgique du jour

La couleur liturgique (depuis `LiturgicalDay.colors`) teinte subtilement toute l'app :

- **Barre latérale gauche** de 3px sur les cartes de célébration
- **Accent de la tab bar** : l'icône de l'onglet actif prend la couleur liturgique
- **En-tête de l'onglet Aujourd'hui** : filet supérieur en couleur liturgique
- **Fond très léger** (optionnel, mode « couleur liturgique immersive ») : le fond parchemin prend une teinte imperceptible de la couleur du jour

Quand l'utilisateur sélectionne une célébration alternative (mémoire facultative), la couleur s'adapte dynamiquement.

### 8.2 Mode sombre

Le mode sombre s'inspire de l'obscurité priante des offices de nuit :
- Fond `#1C1A17` (noir chaud)
- Texte `#E8E2DA` (parchemin inversé)
- Accents liturgiques légèrement plus lumineux pour rester lisibles
- Transition automatique ou manuelle (Réglages > Apparence)

### 8.3 Gestion des choix multiples — Synthèse

Voici comment chaque type de choix Romcal se traduit en UI :

| Donnée Romcal | Situation | Élément UI |
|---|---|---|
| `is_optional: true` | Mémoire facultative disponible | Radio button `◉/○` dans « Célébrations du jour » |
| `optional_celebrations` | Plusieurs mémoires le même jour | Liste de radio buttons (1 seule sélection) |
| Lent + `Rank::Memorial` | Mémoire obligatoire devenue facultative | Radio + badge « Rendue facultative (Carême) » |
| `MassTime` multiples | Plusieurs formulaires de messe | Menu déroulant « Messe du jour / Vigile / Aurore » |
| `LiturgicalCycle` multiples | Lectures de cycles différents | Indicateur `1/2` + navigation `▲▼` sur la lecture |
| `Common` enum | Lectures au choix dans le commun | Menu secondaire « Lectures du commun de… » |
| `allow_similar_rank_items` | Co-célébrations de même rang | Toutes affichées, aucun radio (pas de choix exclusif) |
| `CalendarType` religious | Office propre disponible | Sélecteur « Source » dans l'onglet Heures |
| `ParticularConfig` | Options régionales (Épiphanie, Ascension…) | Toggles dans Réglages > Calendrier & Lieu |

### 8.4 Accessibilité

- **Dynamic Type** : toute la typographie suit les réglages iOS
- **VoiceOver** : labels sémantiques sur chaque élément interactif
- **Contraste** : WCAG AA minimum, AAA visé pour le texte principal
- **Réduction de mouvement** : respecte `UIAccessibility.isReduceMotionEnabled`

### 8.5 Offline-first

L'app fonctionne intégralement hors ligne :
- Les données de calendrier sont pré-calculées localement via Romcal (WASM ou Rust natif via FFI Swift)
- Les textes liturgiques sont embarqués (ou téléchargés une fois puis mis en cache)
- Seuls les commentaires patristiques et les fonctionnalités « Plus » nécessitent une connexion ponctuelle

---

## 9. Architecture technique (résumé)

```
┌─────────────────────────────────────────────────────┐
│                    Cella iOS App                     │
│                                                     │
│  ┌───────────────────────────────────────────────┐  │
│  │              SwiftUI Views                    │  │
│  │  Aujourd'hui · Lectures · Heures · Plus       │  │
│  └──────────────────────┬────────────────────────┘  │
│                         │                           │
│  ┌──────────────────────▼────────────────────────┐  │
│  │            ViewModels / State                 │  │
│  │  DayViewModel · ReadingsVM · HoursVM · ...    │  │
│  │  Gère les choix utilisateur (célébration,     │  │
│  │  formulaire, lectures alternatives)            │  │
│  └──────────────────────┬────────────────────────┘  │
│                         │                           │
│  ┌──────────────────────▼────────────────────────┐  │
│  │          Romcal Swift Binding                 │  │
│  │  RomcalService : interface Swift vers le      │  │
│  │  moteur Rust (via FFI / UniFFI)               │  │
│  │                                               │  │
│  │  • generateLiturgicalCalendar(year:)          │  │
│  │  • generateMassCalendar(year:)                │  │
│  │  • generateHoursCalendar(year:)               │  │
│  │  • dateOf(id:, year:)                         │  │
│  │  • liturgicalDayOf(id:, year:)                │  │
│  │  • massesOf(id:, year:)                       │  │
│  │  • hoursOf(id:, year:)                        │  │
│  │  • searchMartyrolog(query:)                   │  │
│  └──────────────────────┬────────────────────────┘  │
│                         │                           │
│  ┌──────────────────────▼────────────────────────┐  │
│  │          Romcal Core (Rust)                   │  │
│  │  Moteur liturgique compilé en bibliothèque    │  │
│  │  native pour iOS (via cargo-lipo / UniFFI)    │  │
│  │                                               │  │
│  │  Layer 1: Liturgical Calendar                 │  │
│  │  Layer 2: Mass Calendar                       │  │
│  │  Layer 2: Hours Calendar                      │  │
│  │                                               │  │
│  │  Bundled data: definitions + resources        │  │
│  └───────────────────────────────────────────────┘  │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### Correspondance API Romcal → Écrans Cella

| Écran Cella | API Romcal utilisée |
|---|---|
| Onglet Aujourd'hui | `generate_liturgical_calendar()` → `LiturgicalDay` avec `optional_celebrations`, `colors`, `rank`, `season` |
| Sélection célébration | `MassContext.optional_celebrations: Vec<CelebrationSummary>` |
| Oraison du jour | `masses_of(id, year)` → `MassContent[Collect]`, `MassContent[PrayerOverTheOfferings]`, `MassContent[PrayerAfterCommunion]` |
| Mémoire du jour | `get_martyrology_entry(id)` → `MartyrologyEntry` (biography, titles, dates) |
| Onglet Lectures | `generate_mass_calendar()` → `MassContent` par `MassPart` (Reading1, Psalm, Gospel…) |
| Choix formulaire | `MassTime` enum (DayMass, VigillMass, NightMass…) |
| Lectures alternatives | `MassCycleDefinition` par `LiturgicalCycle` (YearA/B/C, Year1/2) |
| Onglet Heures | `generate_hours_calendar()` + `hours_of(id, year)` |
| Cycle psautier | `LiturgicalDay.psalter_week` (Week_1…Week_4) |
| Réglages calendrier | `Romcal::new(Preset { calendar, locale, epiphany_on_sunday, … })` |
| Réglages communauté | `Preset.calendar` avec `CalendarType::GeneralCommunity` / `RegionalCommunity` |

---

## 10. Micro-interactions et détails d'élégance

### 10.1 Transitions entre jours

Le swipe horizontal entre les jours utilise une transition fluide et douce, avec un léger fondu de la couleur liturgique. Pas de *bounce* agressif — le mouvement est calme, comme un tournement de page.

### 10.2 Marquage des offices priés

Dans la grille « Tous les offices » de l'onglet Aujourd'hui :
- Un office **prié** (ouvert et scrollé jusqu'à la fin) reçoit une coche discrète `✓`
- L'office **suggéré** reçoit un point `●`
- Le suivi est purement local, aucune gamification — pas de « streak », pas de score

### 10.3 Respiration typographique

Les lectures et les psaumes utilisent un interligne généreux (1.6–1.8) et des marges latérales amples. Le texte ne touche jamais les bords. Les versets sont clairement séparés par un léger espace vertical.

### 10.4 Lettrines

Les lectures commencent par une **lettrine** de 3 lignes en Cormorant Garamond, dans la couleur liturgique du jour — un rappel des manuscrits enluminés.

### 10.5 Silence au lancement

L'écran de lancement n'est pas un splash screen coloré. C'est un fond parchemin uni avec une fine croix centrée, pendant 0.3 secondes. L'entrée dans l'app est un seuil, pas un spectacle.

---

## 11. Synthèse des écrans

| # | Écran | Accès | Contenu principal |
|---|---|---|---|
| 1 | Aujourd'hui | Tab 1 | Identité liturgique du jour, office suggéré, sélection célébration, oraison, mémoire, grille des offices |
| 2 | Lectures | Tab 2 | Lectures de la messe (1re lecture, psaume, 2e lecture, évangile), commentaire patristique |
| 3 | Heures — Liste | Tab 3 | Liste des 7 offices, sélection source, indication psautier |
| 4 | Heures — Office | Tab 3 → tap | Texte complet d'un office (hymne, psaumes, lecture, cantique, oraison) |
| 5 | Plus | Tab 4 | Explorer (Bible, Prières, Messe.info, Podcasts, Agenda), Réglages, À propos |
| 6 | Qu'est-ce que Cella ? | Tab 4 → À propos | Explication du nom, spiritualité de l'app |
| 7 | Calendrier & Lieu | Tab 4 → Réglages | Région, pays, diocèse, options régionales, calcul de Pâques |
| 8 | Communauté religieuse | Tab 4 → Réglages | Sélection communauté, province, office par défaut |
| 9 | Apparence | Tab 4 → Réglages | Mode clair/sombre/auto, taille de police |
| 10 | Calendrier mensuel | Tap date (Tab 1) | Vue mois avec couleurs liturgiques, navigation rapide |

---

*Ce document constitue la vision de design de Cella. Chaque détail est pensé pour que l'app soit un lieu de prière — silencieux, beau, profond — tout en exposant toute la richesse liturgique que Romcal rend possible.*
