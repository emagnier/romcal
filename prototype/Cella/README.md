# Cella — iOS Prototype

Prototype fonctionnel SwiftUI de l'app Cella, compagnon de prière liturgique construit sur Romcal.

## Ouverture dans Xcode

1. Ouvrir `Package.swift` dans Xcode
2. Sélectionner un simulateur iOS 17+
3. Les previews SwiftUI sont disponibles sur chaque vue

## Structure

```
Sources/
├── CellaApp.swift              # Point d'entrée, écran de lancement
├── Theme/
│   └── CellaTheme.swift        # Couleurs, typographie, styles
├── Models/
│   ├── LiturgicalModels.swift  # Modèles de données liturgiques
│   └── MockData.swift          # Données de test (Carême, jeudi)
└── Views/
    ├── ContentView.swift       # Tab bar principale (4 onglets)
    ├── Today/
    │   └── TodayView.swift     # Onglet 1 — Aperçu du jour
    ├── Readings/
    │   └── ReadingsView.swift  # Onglet 2 — Lectures + commentaire
    ├── Hours/
    │   ├── HoursListView.swift # Onglet 3 — Liste des offices
    │   └── HoursDetailView.swift # Détail d'un office complet
    └── More/
        ├── MoreView.swift              # Onglet 4 — Menu principal
        ├── AboutCellaView.swift        # Signification de Cella
        ├── CalendarSettingsView.swift   # Calendrier & Lieu
        ├── CommunitySettingsView.swift  # Communauté religieuse
        └── AppearanceSettingsView.swift # Thème, taille, couleurs
```

## Écrans

| # | Écran | Description |
|---|---|---|
| 1 | Aujourd'hui | Jour liturgique, office suggéré, sélection célébration (radio buttons), oraison, mémoire du martyrologe |
| 2 | Lectures | Lectures de la messe avec lettrine, psaume, évangile + commentaire patristique |
| 3 | Heures (liste) | 7 offices avec sélecteur de source (romain / bénédictin / dominicain / Jérusalem) |
| 4 | Heures (détail) | Office complet avec mode « Prier pas à pas » |
| 5 | Plus | Bible, Prières, Messe.info, Podcasts, Agenda (à venir) + Réglages |
| 6 | Qu'est-ce que Cella ? | Double signification : cellule monastique + cellier du Cantique |
| 7 | Calendrier & Lieu | Région → Pays → Diocèse, options régionales, calcul de Pâques |
| 8 | Communauté | Choix communauté religieuse + province + office par défaut |
| 9 | Apparence | Thème clair/sombre/auto, taille texte, couleur liturgique immersive |
