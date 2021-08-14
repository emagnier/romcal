import { Locale } from '@romcal/types/locale';

export const locale: Locale = {
  key: 'fr',

  seasons: {
    advent: {
      season: 'Temps de l’Avent',
      weekday:
        '$t(weekdays:{{dow}}, capitalize) de la $t(ordinals:{{week}}, { "context": "feminine" }) semaine de l’Avent',
      sunday: '$t(ordinals:{{week}}, capitalize) dimanche de l’Avent',
      privileged_weekday: '{{day}} $t(months:11)',
    },

    christmas_time: {
      season: 'Temps de Noël',
      day: '$t(weekdays:{{dow}}, capitalize) dans le Temps de Noël',
      octave: '{{count}}ᵉ jour dans l’Octave de la Nativité',
      second_sunday_after_christmas: 'Deuxième dimanche après la Nativité',
      before_epiphany: '{{day}} $t(months:0)',
      after_epiphany: '$t(weekdays:{{dow}}, capitalize) après l’Épiphanie',
    },

    ordinary_time: {
      season: 'Temps Ordinaire',
      weekday:
        '$t(weekdays:{{dow}}, capitalize) de la $t(ordinals:{{week}}, { "context": "feminine" }) semaine du Temps Ordinaire',
      sunday: '$t(ordinals:{{week}}, capitalize) $t(weekdays:0) du Temps Ordinaire',
    },

    lent: {
      season: 'Temps du Carême',
      weekday:
        '$t(weekdays:{{dow}}, capitalize) de la $t(ordinals:{{week}}, { "context": "feminine" }) semaine de Carême',
      sunday: '$t(ordinals:{{week}}, capitalize) dimanche de Carême',
      day_after_ash_wed: '$t(weekdays:{{dow}}, capitalize) après les Cendres',
      holy_week_day: '$t(weekdays:{{dow}}, capitalize) Saint',
    },

    paschal_triduum: {
      season: 'Triduum Pascal',
    },

    easter_time: {
      season: 'Temps Pascal',
      weekday:
        '$t(weekdays:{{dow}}, capitalize) de la $t(ordinals:{{week}}, { "context": "feminine" }) semaine de Pâques',
      sunday: '$t(ordinals:{{week}}, capitalize) dimanche de Pâques',
      octave: '$t(weekdays:{{dow}}, capitalize) dans l’Octave de Pâques',
    },
  },

  periods: {
    epiphany: 'Épiphanie',
    holy_week: 'Semaine Sainte',
  },

  ranks: {
    solemnity: 'Solennité',
    sunday: 'Dimanche',
    feast: 'Fête',
    memorial: 'Mémoire',
    weekday: 'Férie',
  },

  weekdays: {
    0: 'dimanche',
    1: 'lundi',
    2: 'mardi',
    3: 'mercredi',
    4: 'jeudi',
    5: 'vendredi',
    6: 'samedi',
  },

  months: {
    0: 'janvier',
    1: 'février',
    2: 'mars',
    3: 'avril',
    4: 'mai',
    5: 'juin',
    6: 'juillet',
    7: 'août',
    8: 'septembre',
    9: 'octobre',
    10: 'novembre',
    11: 'décembre',
  },

  colors: {
    black: 'noir',
    gold: 'doré',
    green: 'vert',
    purple: 'violet',
    red: 'rouge',
    rose: 'rose',
    white: 'blanc',
  },

  ordinals: {
    '1': 'premier',
    '1_feminine': 'première',
    '2': 'deuxième',
    '3': 'troisième',
    '4': 'quatrième',
    '5': 'cinquième',
    '6': 'sixième',
    '7': 'septième',
    '8': 'huitième',
    '9': 'neuvième',
    '10': 'dixième',
    '11': 'onzième',
    '12': 'douzième',
    '13': 'treizième',
    '14': 'quatorzième',
    '15': 'quinzième',
    '16': 'seizième',
    '17': 'dix-septième',
    '18': 'dix-huitième',
    '19': 'dix-neuvième',
    '20': 'vingtième',
    '21': 'vingt et unième',
    '22': 'vingt-deuxième',
    '23': 'vingt-troisième',
    '24': 'vingt-quatrième',
    '25': 'vingt-cinquième',
    '26': 'vingt-sixième',
    '27': 'vingt-septième',
    '28': 'vingt-huitième',
    '29': 'vingt-neuvième',
    '30': 'trentième',
    '31': 'trente et unième',
    '32': 'trente-deuxième',
    '33': 'trente-troisième',
    '34': 'trente-quatrième',
  },

  names: {
    all_saints: 'Tous les Saints',
    annunciation: 'Annonciation du Seigneur',
    ascension: 'Ascension du Seigneur',
    ash_wednesday: 'Mercredi des Cendres',
    assumption: 'Assomption de la vierge Marie',
    baptism_of_the_lord: 'Baptême du Seigneur',
    christ_the_king_sunday: 'Trente-quatrième et dernier dimanche - Le Christ, Roi de l’Univers',
    christmas: 'Nativité du Seigneur',
    corpus_christi: 'Le Saint-Sacrement du Corps et du Sang du Christ',
    divine_mercy_sunday: 'Deuxième dimanche de Pâques ou de la Divine Miséricorde',
    easter_sunday: 'Dimanche de Pâques - La résurrection du Seigneur',
    epiphany: 'Épiphanie du Seigneur',
    exaltation_of_the_holy_cross: 'La Croix Glorieuse',
    good_friday: 'Vendredi Saint',
    holy_family: 'La Sainte Famille',
    holy_saturday: 'Samedi Saint',
    holy_thursday: 'Jeudi Saint',
    immaculate_conception_of_mary: 'Immaculée Conception de la vierge Marie',
    immaculate_heart_of_mary: 'Cœur immaculé de Marie',
    joseph_spouse_of_mary: 'Saint Joseph, Époux de la vierge Marie',
    mary_mother_of_god: 'Sainte Marie, Mère de Dieu',
    most_sacred_heart_of_jesus: 'Sacré-Cœur de Jésus',
    nativity_of_john_the_baptist: 'Nativité de Saint Jean le Baptiste',
    palm_sunday: 'Dimanche des Rameaux et de la Passion',
    pentecost_sunday: 'Pentecôte',
    peter_and_paul_apostles: 'Saint Pierre et Saint Paul, apôtres',
    presentation_of_the_lord: 'Présentation du Seigneur au Temple',
    sunday_of_the_word_of_god: 'Troisième dimanche du Temps Ordinaire, ou de la Parole de Dieu',
    thursday_of_the_lord_s_supper: 'Mémoire de la Cène du Seigneur',
    transfiguration: 'Transfiguration du Seigneur',
    trinity_sunday: 'La Sainte Trinité',
    adalbert_of_prague_bishop: 'Saint Adalbert, évêque de Prague et martyr (✝ 997)',
    adalbert_of_prague_bishop_patron_of_poland:
      'Saint Adalbert, évêque de Prague, martyr et patron de la Pologne (✝ 997)',
    adelaide_of_burgundy: 'Sainte Adélaîde, impératrice (✝ 999)',
    adelphus_bishop: 'Saint Adelphe, évêque (Vème s.)',
    agatha_of_sicily_virgin: 'Sainte Agathe, vierge et martyre en Sicile (✝ 251)',
    agnes_of_rome_virgin: 'Sainte Agnès, vierge et martyre (✝ v. 304)',
    albert_the_great_bishop:
      'Saint Albert le Grand, frère prêcheur, évêque de Ratisbonne, docteur de l’Église (✝ 1280)',
    all_souls: 'Commémoration de tous les fidèles défunts',
    all_the_saints_of_the_diocese_of_paris: 'Tous les Saints du diocèse de Paris',
    all_the_saints_of_the_diocese_of_saint_denis: 'Tous les Saints du diocèse de Saint-Denis',
    aloysius_gonzaga_religious: 'Saint Louis de Gonzague, religieux Jésuite (✝ 1591)',
    alphonsus_liguori_bishop: 'Saint Alphonse-Marie de Liguori, évêque et docteur de l’Église',
    amandus_first_bishop_of_strasbourg: 'Saint Amand, premier évêque de Strasbourg (✝ v. 346)',
    amarin_abbot: 'Saint Amarin, abbé et martyr (✝ 674 ou 676)',
    amand_of_maastricht_bishop:
      'Saint Amand d’Elnone, Missionnaire, évêque de Maastricht (✝ v. 676)',
    ambrose_of_milan_bishop: 'Saint Ambroise, évêque de Milan et docteur de l’Église (✝ 397)',
    andre_bauer_martyr: 'Saint André Bauer, martyr (✝ 1900)',
    andre_bessette_religious: 'Saint frère André Bessette, religieux (✝ 1937)',
    andre_grasset_priest:
      'Bienheureux André Grasset, prêtre, canadien martyr de la révolution française (✝ 1792)',
    andrew_apostle: 'Saint André, apôtre',
    andrew_apostle_patron_of_scotland: 'Saint André, apôtre et patron de l’Écosse',
    andrew_dung_lac_priest_and_companions_martyrs:
      'Saint André Dung Lac, prêtre, et ses compagnons, martyrs (✝ entre 1745 et 1862)',
    andrew_kim_tae_gon_priest_paul_chong_ha_sang_and_companions_martyrs:
      'Saint André Kim Taegon, prêtre, et Paul Chong Ha-sang et ses compagnons, martyrs en Corée (XIXème s.)',
    andrew_zorard_of_nitra_and_benedict_of_skalka_hermits:
      'Saints André Svorad († 1009) et Benoît Stojislav († 1012), ermites',
    angela_merici_virgin:
      'Sainte Angèle Mérici, religieuse, fondatrice de la Compagnie de Sainte Ursule de Brescia (✝ 1540)',
    anselm_of_canterbury_bishop:
      'Saint Anselme de Cantorbéry, évêque, docteur de l’Église (✝ 1109)',
    ansgar_of_hamburg_bishop: 'Saint Anschaire de Brême, évêque (✝ 865)',
    anthony_mary_claret_bishop: 'Saint Antoine-Marie Claret, évêque (✝ 1870)',
    anthony_of_egypt_abbot: 'Saint Antoine le Grand, ermite en Égypte (✝ 356)',
    anthony_of_padua_priest: 'Saint Antoine, prêtre et docteur de l’Église (✝ 1231)',
    anthony_zaccaria_priest: 'Saint Antoine-Marie Zaccaria, prêtre (✝ 1539)',
    apollinaris_of_ravenna_bishop: 'Saint Apollinaire, évêque et martyr (IIème s.)',
    arbogast_of_strassburg_bishop: 'Saint Arbogast, évêque (✝ 678)',
    arbogast_of_strassburg_bishop_patron_of_the_diocese_of_strasbourg:
      'Saint Arbogast, évêque, patron du diocèse de Strasbourg (✝ 678)',
    athanasius_of_alexandria_bishop: 'Saint Athanase, évêque, docteur et père de l’Église (✝ 373)',
    augustine_of_canterbury_bishop: 'Saint Augustin, évêque de Cantorbéry en Angleterre (✝ 604)',
    augustine_of_hippo_bishop: 'Saint Augustin, évêque d’Hippone, docteur de l’Église (✝ 430)',
    augustine_zhao_rong_priest_and_companions_martyrs:
      'Saints Augustin Zhao Rong et ses compagnons, martyrs en Chine (entre 1648 et 1930)',
    aurelia_virgin: 'Sainte Aurélie, vierge',
    barnabas_apostle: 'Saint Barnabé, apôtre',
    bartholomew_apostle: 'Saint Barthélemy, apôtre',
    basil_the_great_and_gregory_nazianzen_bishops:
      'Saints Basile le Grand (✝ 379) et Grégoire de Naziance (✝ 390), évêques et docteurs de l’Église',
    bede_the_venerable_priest: 'Saint Bède le Vénérable, prêtre et docteur de l’Église (✝ 735)',
    benedict_of_aniane_abbot: 'Saint Benoît d’Aniane, abbé (✝ 820)',
    benedict_of_nursia_abbot:
      'Saint Benoît de Nursie, Patriarche des moines d’Occident, fondateur de l’ordre des Bénédictins (✝ v. 547)',
    benedict_of_nursia_abbot_patron_of_europe:
      'Saint Benoît de Nursie, Patriarche des moines d’Occident, fondateur de l’ordre des Bénédictins et patron de l’Europe (✝ v. 547)',
    bernadette_soubirous_virgin: 'Sainte Bernadette Soubirous, vierge (✝ 1879)',
    bernard_of_clairvaux_abbot: 'Saint Bernard de Clairvaux, abbé, docteur de l’Église (✝ 1153)',
    bernardine_of_siena_priest: 'Saint Bernardin de Sienne, prêtre (✝ 1444)',
    blaise_of_sebaste_bishop: 'Saint Blaise de Sébaste, évêque et martyr (✝ 316)',
    blessed_martyrs_of_paris: 'Bienheureux martyrs de Paris († du 2 au 6 septembre 1792)',
    bonaventure_of_bagnoregio_bishop:
      'Saint Bonaventure, évêque d’Albano et docteur de l’Église (✝ 1274)',
    boniface_of_mainz_bishop: 'Saint Boniface, évêque et martyr (✝ 754)',
    bridget_of_sweden_religious:
      'Sainte Brigitte de Suède, veuve puis religieuse, fondatrice de l’ordre du Saint-Sauveur (✝ 1373)',
    bridget_of_sweden_religious_copatroness_of_europe:
      'Sainte Brigitte de Suède, veuve puis religieuse, fondatrice de l’ordre du Saint-Sauveur, co-patronne de l’Europe (✝ 1373)',
    brigid_of_kildare_virgin: 'Saint Brigide, abbesse (✝ 523)',
    bruno_of_cologne_priest: 'Saint Bruno, prêtre, fondateur des Chartreux (✝ 1101)',
    caesarius_of_arles_bishop: 'Saint Césaire d’Arles, évêque (✝ 542)',
    cajetan_of_thiene_priest: 'Saint Gaétan de Thienne, prêtre (✝ 1547)',
    callistus_i_pope: 'Saint Calixte Ier, pape et martyr (✝ 222)',
    camillus_de_lellis_priest: 'Saint Camille de Lellis, prêtre (✝ 1614)',
    carmelites_of_compiegne_virgins_and_martyrs:
      'Bienheureuses Carmélites de Compiègne : Mère Thérèse et ses 15 compagnes, martyres (guillotinées en 1794)',
    casimir_of_poland: 'Saint Casimir (✝ 1484)',
    catherine_laboure_virgin: 'Sainte Catherine Labouré, vierge († 1876)',
    catherine_of_alexandria_virgin: 'Sainte Catherine d’Alexandrie, vierge et martyre (IVème s.)',
    catherine_of_saint_augustine_de_simon_de_longpre_virgin:
      'Bienheureuse Catherine de Saint-Augustin, religieuse hospitalière de la Miséricorde (✝ 1668)',
    catherine_of_siena_virgin: 'Sainte Catherine de Sienne, vierge et docteur de l’Église (✝ 1380)',
    catherine_of_siena_virgin_copatroness_of_europe:
      'Sainte Catherine de Sienne, vierge, docteur de l’Église, co-patronne de l’Europe (✝ 1380)',
    catherine_of_siena_virgin_copatroness_of_italy_and_europe:
      'Sainte Catherine de Sienne, vierge, docteur de l’Église, co-patronne de l’Italie et de l’Europe (✝ 1380)',
    cecilia_of_rome_virgin: 'Sainte Cécile, vierge et martyre à Rome (✝ 230)',
    ceran_of_paris_bishop: 'Saint Céran, évêque de Paris au 7e siècle',
    chair_of_saint_peter_the_apostle: 'Chaire de Saint Pierre, apôtre',
    charles_borromeo_bishop: 'Saint Charles Borromée, Archevêque de Milan (✝ 1584)',
    charles_de_foucauld:
      'Bienheureux Charles de Foucauld, prêtre, ermite et Missionnaire au Sahara († 1916)',
    charles_lwanga_and_companions_martyrs:
      'Saints Charles Lwanga et ses douze compagnons, martyrs (✝ 618)',
    christopher_magallanes_priest_and_companions_martyrs:
      'Saints Cristóbal Magallanes, prêtre, et ses 24 compagnons, martyrs Mexicains (✝ 1927)',
    clare_of_assisi_virgin: 'Sainte Claire, vierge',
    clement_i_pope: 'Saint Clément Ier, pape et martyr (✝ 97)',
    clotilde_of_burgundy: 'Sainte Clotilde, reine des Francs (✝ 545)',
    columba_of_iona_abbot: 'Saint Colomba, religieux (✝ 615)',
    columba_of_iona_abbot_copatron_of_ireland:
      'Saint Colomba, religieux, co-patron de l’Irlande (✝ 615)',
    conversion_of_saint_paul_the_apostle: 'Conversion de Saint Paul, apôtre',
    cornelius_i_pope_and_cyprian_of_carthage_bishop_martyrs:
      'Saints martyrs Corneille, pape, et Cyprien, évêque (IIIème s.)',
    cosmas_of_cilicia_and_damian_of_cilicia_martyrs: 'Saints Côme et Damien, martyrs (IIIème s.)',
    cyril_of_alexandria_bishop: 'Saint Cyrille, évêque d’Alexandrie et docteur de l’Église (✝ 444)',
    cyril_of_jerusalem_bishop:
      'Saint Cyrille de Jérusalem, évêque de Jérusalem, docteur de l’Église (✝ 387)',
    cyril_the_philosopher_monk_and_methodius_of_thessaloniki_bishop:
      'Saints Cyrille, Moine, et Méthode, évêque, apôtres des Slaves au IXème siècle',
    cyril_the_philosopher_monk_and_methodius_of_thessaloniki_bishop_copatrons_of_europe:
      'Saints Cyrille, Moine, et Méthode, évêque, apôtres des Slaves au IXème siècle, co-patrons de l’Europe',
    damasus_i_pope: 'Saint Damase Ier, pape (✝ 384)',
    damien_de_veuster_priest: 'Saint Père Damien, prêtre et Missionnaire Picpus (✝ 1889)',
    daniel_brottier_priest:
      'Binheureux Daniel Brottier, prêtre, apôtre des Orphelins d’Auteuil († 1936 à Paris)',
    dedication_of_the_basilica_of_saint_mary_major: 'Dédicace de la basilique Sainte-Marie Majeure',
    dedication_of_the_basilicas_of_saints_peter_and_paul_apostles:
      'Dédicace des basiliques de Saint Pierre et Saint Paul, apôtres, à Rome',
    dedication_of_the_cathedral_of_paris: 'Dédicace de la cathédrale de Paris',
    dedication_of_the_cathedral_of_saint_denis: 'Dédicace de la cathédrale de Saint-Denis',
    dedication_of_the_cathedral_of_strasbourg: 'Dédicace de la cathédrale de Strasbourg',
    dedication_of_the_lateran_basilica: 'Dédicace de la Basilique du Latran',
    denis_of_paris_bishop_patron_of_the_diocese_of_paris:
      'Saint Denis, martyr, premier évêque, patron du diocèse',
    denis_of_paris_bishop_patron_of_the_diocese_of_saint_denis:
      'Saint Denis, évêque et martyr, patron de la ville et du diocèse de Saint-Denis',
    denis_of_paris_bishop_and_companions_martyrs:
      'Saint Denis, évêque, et ses compagnons, martyrs à Paris (IIIème s.)',
    dina_belanger_virgin:
      'Bienheureuse Dina Bélanger, religieuse de la congrégation des Sœurs de Jésus-Marie (✝ 1929)',
    dominic_de_guzman_priest:
      'Saint Dominique de Guzman, prêtre, fondateur de l’Ordre des Frères prêcheurs (✝ 1221)',
    elijah_prophet: 'Saint Élie, prophète (IXe siècle av. J.-C.)',
    elizabeth_of_hungary_religious: 'Sainte Élisabeth de Hongrie (✝ 1231)',
    elizabeth_of_portugal: 'Sainte Élisabeth du Portugal, reine (✝ 1336)',
    eloi_of_noyon_bishop: 'Saint Éloi, évêque',
    emilie_tavernier_gamelin_religious:
      'Bienheureuse Émilie Tavernier-Gamelin, religieuse, fondatrice des Sœurs de la Providence de Montréal (✝ 1851)',
    ephrem_the_syrian_deacon: 'Saint Ephrem, diacre et docteur de l’Église, (✝ 373)',
    eucharius_of_treves_bishop: 'Saint Euchaire, évêque (IVème s.)',
    eugene_de_mazenod_bishop:
      'Saint Eugène de Mazenod, fondateur des Oblats de Marie-Immaculée, évêque de Marseille (✝ 1861)',
    eugenie_and_attale_of_alsace: 'Sainte Eugénie et Sainte Attale, vierges (VIIIème s.)',
    eusebius_of_vercelli_bishop: 'Saint Eusèbe de Verceil, évêque (✝ 371)',
    fabian_i_pope: 'Saint Fabien, pape et martyr (✝ 250)',
    faustina_kowalska_virgin: 'Sainte Faustina Kowalska (✝ 1938)',
    fidelis_of_sigmaringen_priest: 'Saint Fidèle de Sigmaringen, prêtre et martyr (✝ 1622)',
    first_martyrs_of_the_holy_roman_church: 'Premiers martyrs de l’Église de Rome (✝ 64)',
    frances_of_rome_religious: 'Sainte Françoise Romaine, religieuse (✝ 1440)',
    francis_de_sales_bishop:
      'Saint François de Sales, évêque de Genève et docteur de l’Église (✝ 1622)',
    francis_of_assisi: 'Saint François d’Assise, fondateur de l’ordre des Frères mineurs (✝ 1226)',
    francis_of_assisi_patron_of_italy:
      'Saint François d’Assise, fondateur de l’ordre des Frères mineurs, patron de l’Italie (✝ 1226)',
    francis_of_paola_hermit:
      'Saint François de Paule, ermite, fondateur de l’ordre des Minimes (✝ 1507)',
    francis_xavier_priest: 'Saint François-Xavier, prêtre, Jésuite Missionnaire (✝ 1552)',
    francois_de_montmorency_laval_bishop:
      'Saint François de Laval, premier évêque de Québec (✝ 1708)',
    frederic_janssoone_priest: 'Bienheureux Frédéric Janssoone, prêtre franciscain (✝ 1916)',
    frederic_ozanam:
      'Bienheureux Frédéric Ozonaman, fondateur de la Société de Saint-Vincent-de-Paul († 1853)',
    fridolin_of_sackingen_monk: 'Saint Fridolin, moine missionnaire (✝ 540)',
    florentius_of_strasbourg_bishop: 'Saint Florent, évêque (VIIème s.)',
    gall_of_switzerland_abbot: 'Saint Gall, abbé et missionnaire (✝ 641 ou 646)',
    genevieve_of_paris_virgin: 'Sainte Geneviève, vierge (✝ 500)',
    george_of_lydda_martyr: 'Saint Georges, martyr (✝ 303)',
    george_of_lydda_martyr_patron_of_england:
      'Saint Georges, martyr et patron de l’Angleterre (✝ 303)',
    germain_of_paris_bishop: 'Saint Germain, évêque de Paris († 576)',
    gertrude_of_nivelles_abbess: 'Sainte Gertrude de Nivelles, abbesse (✝ 659)',
    gertrude_the_great_virgin: 'Sainte Gertrude, vierge moniale (✝ 1301)',
    gregory_i_the_great_pope: 'Saint Grégoire le Grand, pape et docteur de l’Église (✝ 604)',
    gregory_of_narek_abbot: 'Saint Grégoire de Narek, abbé et docteur de l’Église',
    gregory_vii_pope: 'Saint Grégoire VII, pape (✝ 1085)',
    guardian_angels: 'Saints Anges gardiens',
    hedwig_of_silesia_religious: 'Sainte Edwige de Silésie, veuve puis religieuse (✝ 1243)',
    henry_ii_emperor: 'Saint Henri, empereur germanique (✝ 1024)',
    henry_of_finland_bishop: 'Saint Henri, évêque et martyr (✝ 1156)',
    hilary_of_poitiers_bishop: 'Saint Hilaire de Poitiers, évêque et docteur de l’Église (✝ 367)',
    holy_innocents_martyrs: 'Saints Innocents, martyrs (Ier s.)',
    hubert_of_liege_bishop: 'Saint Hubert, évêque (✝ 727)',
    ignatius_of_antioch_bishop:
      'Saint Ignace d’Antioche, évêque et martyr, père et docteur de l’Église (✝ 115)',
    ignatius_of_loyola_priest:
      'Saint Ignace de Loyola, prêtre, fondateur de la Compagnie de Jésus (✝ 1556)',
    innocent_v_pope: 'Bienheureux Innocent V, pape († 1276)',
    irenaeus_of_lyon_bishop: 'Saint Irénée, évêque et martyr (✝ v. 201)',
    isabelle_of_france_virgin:
      'Bienheureuse Isabelle de France, sœur de saint Louis, vierge († 1270)',
    isidore_of_seville_bishop:
      'Saint Isidore de Séville, docteur de l’Église, évêque et Confesseur (✝ 636)',
    ivo_of_kermartin_priest: 'Saint Yves Hélory, prêtre et juge en Bretagne (✝ 1303)',
    james_apostle: 'Saint Jacques le Majeur, apôtre (✝ 44)',
    jane_frances_de_chantal_religious: 'Sainte Jeanne-Françoise de Chantal, religieuse (✝ 1641)',
    januarius_i_of_benevento_bishop: 'Saint Janvier, évêque de Bénévent et martyr (✝ 305)',
    jean_georges_rehm_priest_and_martyr: 'Bienheureux Jean-Georges Rehm, prêtre et martyr (✝ 1794)',
    jerome_emiliani: 'Saint Jérôme Émilien, fondateur (✝ 1537)',
    jerome_of_stridon_priest: 'Saint Jérôme, père et docteur de l’Église (✝ 420)',
    joachim_and_anne_parents_of_mary:
      'Saints Anne et Joachim, parents de la vierge Marie (Ier siècle)',
    joachim_and_anne_patroness_of_the_province_of_quebec_parents_of_mary:
      'Saints Anne, patronne de la province civile de Québec, et Joachim, parents de la vierge Marie (Ier siècle)',
    joan_of_arc_virgin_copatroness_of_france:
      'Sainte Jeanne d’Arc, vierge, co-patronne de la France (✝ 1431)',
    john_apostle: 'Saint Jean, apôtre et évangéliste',
    john_baptist_de_la_salle_priest:
      'Saint Jean-Baptiste de La Salle, prêtre et fondateur des Frères des Écoles Chrétiennes (✝ 1719)',
    john_berchmans_religious: 'Saint Jean Berchmans, Jésuite Belge (✝ 1621)',
    john_bosco_priest: 'Saint Jean Bosco, prêtre, fondateur et Éducateur (✝ 1888)',
    john_chrysostom_bishop:
      'Saint Jean Chrysostome, évêque de Constantinople et docteur de l’Église (✝ 407)',
    john_damascene_priest: 'Saint Jean Damascène, prêtre et docteur de l’Église (✝ 749)',
    john_de_brebeuf_isaac_jogues_priests_and_companions_martyrs:
      'Saints Jean de Brébeuf, Isaac Jogues, prêtres Jésuites, et leurs compagnons, martyrs (XVIIème siècle)',
    john_de_brebeuf_isaac_jogues_priests_and_companions_martyrs_copatrons_of_canada:
      'Saints Jean de Brébeuf, Isaac Jogues, prêtres Jésuites, et leurs compagnons, martyrs, co-patrons du Canada (XVIIème siècle)',
    john_eudes_priest: 'Saint Jean Eudes, prêtre (✝ 1680)',
    john_fisher_bishop_and_thomas_more_martyrs:
      'Saints Jean Fisher, évêque, et Thomas More, martyrs (✝ 1535)',
    john_i_pope: 'Saint Jean Ier, pape et martyr (✝ 526)',
    john_leonardi_priest: 'Saint Jean Léonardi, prêtre (✝ 1609)',
    john_mary_vianney_priest: 'Saint Jean-Marie Vianney (Curé d’Ars), prêtre (✝ 1859)',
    john_of_capistrano_priest: 'Saint Jean de Capistran, prêtre de l’Ordre des Mineurs (✝ 1456)',
    john_of_god_duarte_cidade_religious:
      'Saint Jean de Dieu, religieux, fondateur des Frères de la Charité (✝ 1550)',
    john_of_kanty_priest: 'Saint Jean de Kenty, prêtre (✝ 1473)',
    john_of_the_cross_priest: 'Saint Jean de la Croix, prêtre et docteur de l’Église (✝ 1591)',
    john_paul_ii_pope: 'Saint Jean-Paul II, pape (✝ 2005)',
    john_xxiii_pope: 'Saint Jean XXIII, pape (✝ 1963)',
    josaphat_kuntsevych_bishop: 'Saint Josaphat Kuntsevych, évêque Basilien et martyr (✝ 1623)',
    joseph_of_calasanz_priest: 'Saint Joseph de Calasanz, prêtre (✝ 1648)',
    joseph_spouse_of_mary_patron_of_canada:
      'Saint Joseph, Époux de la Bienheureuse vierge Marie, patron du Canada',
    joseph_the_worker: 'Saint Joseph, Artisan (Ier s.)',
    josephine_bakhita_virgin: 'Sainte Joséphine Bakhita, vierge et religieuse (✝ 1947)',
    juan_diego_cuauhtlatoatzin: 'Saint Juan Diego Cuauhtlatoatzin (✝ 1548)',
    juliana_of_liege_virgin: 'Sainte Julienne de Cornillon, religieuse Augustine (✝ 1258)',
    julie_billiart_virgin:
      'Sainte Julie Billiart, religieuse, fondatrice de l’Institut des Sœurs de Notre-Dame (✝ 1816)',
    justin_martyr: 'Saint Justin, martyr (✝ 165)',
    kateri_tekakwitha_virgin: 'Sainte Kateri Tekakwitha, vierge Amérindienne (✝ 1680)',
    lambert_of_maastricht_bishop: 'Saint Lambert, évêque et martyr (✝ 705)',
    landry_of_paris_bishop: 'Saint Landry, évêque de Paris au 7e siècle',
    lawrence_of_brindisi_priest:
      'Saint Laurent de Brindisi, prêtre et docteur de l’Église (✝ 1619)',
    lawrence_of_rome_deacon: 'Saint Laurent de Rome, diacre et martyr (✝ 258)',
    lawrence_ruiz_and_companions_martyrs:
      'Saints Laurent Ruiz et 15 compagnons, martyrs à Nagasaki au Japon (✝ v. 1635)',
    leo_i_the_great_pope: 'Saint Léon le Grand, pape et docteur de l’Église',
    leo_ix_pope: 'Saint Léon IX, pape (✝ 540)',
    leodegar_bishop_and_martyr: 'Saint Léger, évêque et martyr (✝ 679 ou 680)',
    louis_grignion_de_montfort_priest: 'Saint Louis-Marie Grignion de Montfort, prêtre (✝ 1716)',
    louis_ix_of_france: 'Saint Louis, roi de France (✝ 1270)',
    louis_zephirin_moreau_bishop: 'Bienheureux Louis Zéphyrin Moreau, évêque (✝ 1901)',
    louise_of_marillac: 'Sainte Louise de Marillac, religieuse (✝ 1660)',
    lucy_of_syracuse_virgin: 'Sainte Lucie, vierge et martyre en Sicile (✝ v. 305)',
    ludan_pilgrim: 'Saint Ludan, pèlerin (✝ 1202)',
    luke_evangelist: 'Saint Luc, évangéliste',
    madeleine_sophie_barat_virgin: 'Sainte Madeleine Sophie Barat, vierge († 1865)',
    marcel_of_paris_bishop: 'Saint Marcel, évêque de Paris († vers 430)',
    marcellinus_of_rome_and_peter_the_exorcist_martyrs:
      'Saints martyrs Marcellin, prêtre, et Pierre, Exorciste (✝ 304)',
    margaret_mary_alacoque_virgin:
      'Sainte Marguerite-Marie Alacoque, Visitandine à Paray-le-Monial (✝ 1690)',
    margaret_of_scotland: 'Sainte Marguerite d’Ecosse (✝ 1093)',
    marguerite_bourgeoys_virgin:
      'Sainte Marguerite Bourgeoys, religieuse, fondatrice de la congrégation des Sœurs de Notre-Dame (✝ 1700)',
    marguerite_dyouville_religious:
      'Sainte Marguerite d’Youville, religieuse, fondatrice des Sœurs de la Charité de Montréal (✝ 1771)',
    maria_goretti_virgin: 'Sainte Maria Goretti, vierge et martyre (✝ 1902)',
    marie_anne_blondin_virgin:
      'Bienheureuse Marie-Anne Blondin, religieuse, fondatrice des Sœurs de Sainte-Anne (✝ 1890)',
    marie_de_la_providence_virgin:
      'Bienheureuse Marie de la Providence (Eugénie Smet), vierge, fondatrice de la congrégration des Auxilliatrices († 1871 à Paris)',
    marie_de_incarnation_religious:
      'Bienheureuse Marie de l‘Incarnation, Carmélite à Pontoise († 1618)',
    marie_eugenie_milleret_virgin:
      'Sainte Marie-Eugénie Milleret, vierge, fondatrice des Sœurs de l’Assomption († 1898 à Paris)',
    marie_leonie_paradis_virgin:
      'Bienheureuse Marie-Léonie Paradis, religieuse, fondatrice de la Congrégation des Petites Sœurs de la Sainte Famille (✝ 1912)',
    marie_of_the_incarnation_guyart_religious:
      'Sainte Marie de l’Incarnation Guyart, Ursuline au Canada (✝ 1672)',
    marie_rose_durocher_virgin:
      'Bienheureuse Marie-Rose Durocher, religieuse, fondatrice des Sœurs des Saints Noms de Jésus et de Marie (✝ 1849)',
    marie_therese_de_soubiran_virgin: 'Bienheureuse Marie-Thérèse de Soubiran, vierge († 1889)',
    mark_evangelist: 'Saint Marc, évangéliste',
    martha_of_bethany_mary_of_bethany_and_lazarus_of_bethany:
      'Saints Marthe, Marie et Lazare, disciples du Christ (Ier s.)',
    martin_de_porres_religious: 'Saint Martin de Porrès, religieux Dominicain à Lima (✝ 1639)',
    martin_i_pope: 'Saint Martin Ier, pape et martyr (✝ 656)',
    martin_of_tours_bishop: 'Saint Martin de Tours, évêque (✝ 397)',
    mary_magdalene: 'Sainte Marie-Madeleine, disciple du Christ (Ier s.)',
    mary_magdalene_de_pazzi_virgin:
      'Sainte Marie-Madeleine de Pazzi, vierge de l’Ordre du Carmel (✝ 1607)',
    mary_refuge_of_sinner: 'Sainte Marie, refuge des pécheurs',
    maternus_of_cologne_and_valerius_of_treves_and_eucharius_first_apostles_of_alsace:
      'Les premiers Apôtres de l‘Église en Alsace : Saints Materne, Valère et Euchaire',
    maurice_of_agaunum_and_companions_martyrs:
      'Saint Maurice et ses companions, martyrs (✝ v. 286)',
    maternus_of_cologne_bishop: 'Saint Materne, évêque (IVème s.)',
    matthew_apostle: 'Saint Matthieu, apôtre et évangéliste',
    matthias_apostle: 'Saint Matthias, apôtre',
    maximilian_kolbe_priest: 'Saint Maximilien-Marie Kolbe, prêtre et martyr (✝ 1941)',
    modeste_andlauer_and_andre_bauer_martyrs:
      'Saints Modeste Andlauer et André Bauer, martyrs (✝ 1900)',
    modeste_andlauer_martyr: 'Saint Modeste Andlauer, martyr (✝ 1900)',
    morandus_of_cluny_monk: 'Saint Morand, moine (✝ v. 1115)',
    michael_gabriel_and_raphael_archangels: 'Saints Michel, Gabriel and Raphaël, archanges',
    monica_of_hippo: 'Sainte Monique, Mère de Saint Augustin d’Hippone (✝ 387)',
    most_holy_name_of_jesus: 'Saint Nom de Jésus',
    most_holy_name_of_mary: 'Le Saint Nom de Marie',
    mutien_marie_wiaux_religious: 'Saint Mutien-Marie Wiaux, Frère des Écoles Chrétiennes (✝ 1917)',
    nativity_of_mary: 'Nativité de la vierge Marie',
    nereus_of_terracina_and_achilleus_of_terracina_martyrs:
      'Saints Nérée et Achillée, martyrs à Rome (✝ v. 304)',
    nicholas_of_myra_bishop: 'Saint Nicolas, évêque de Myre (✝ v. 350)',
    nicholas_barre: 'Bienheureux. Nicolas Barré, prêtre († 1686 à Paris)',
    norbert_of_xanten_bishop: 'Saint Norbert, évêque (✝ 1134)',
    nykyta_budka_and_vasyl_velychkovsky_bishops:
      'Bienheureux Nicétas Budka (✝ 1949) et Vasyl Velychkowsky (✝ 1973), évêques et martyrs',
    odile_of_alsace_abbess: 'Sainte Odile, abbesse',
    odile_of_alsace_abbess_patroness_of_alsace:
      'Sainte Odile, abbesse, patronne de l‘Alsace (✝ v. 720)',
    ouen_bishop: 'Saint Ouen, évêque',
    our_lady_mediatrix_of_all_grace: 'Marie, Médiatrice de toute grâce',
    our_lady_of_fatima: 'Notre-Dame de Fatima',
    our_lady_of_good_counsel: 'Notre-Dame du Bon Conseil',
    our_lady_of_guadalupe: 'Notre-Dame de Guadalupé',
    our_lady_of_guadalupe_patroness_of_the_americas:
      'Notre-Dame de Guadalupé, patronne des Amériques',
    our_lady_of_lourdes: 'Notre-Dame de Lourdes',
    our_lady_of_mount_carmel: 'Notre-Dame du Mont-Carmel',
    our_lady_of_sorrows: 'Notre-Dame des Douleurs',
    our_lady_of_sorrows_patroness_of_slovakia: 'Notre-Dame des Douleurs, patronne de la Slovaquie',
    our_lady_of_the_miraculous_medal: 'Notre-Dame de la Médaille miraculeuse',
    our_lady_of_the_rosary: 'Notre-Dame du Rosaire',
    pancras_of_rome_martyr: 'Saint Pancrace, martyr à Rome (✝ v. 304)',
    passion_of_saint_john_the_baptist: 'martyre de Saint Jean-Baptiste',
    patrick_of_ireland_bishop: 'Saint Patrick, évêque (✝ 461)',
    patrick_of_ireland_bishop_patron_of_ireland:
      'Saint Patrick, évêque, patron de l’Irlande (✝ 461)',
    paul_miki_and_companions_martyrs:
      'Saints Paul Miki et ses compagnons, martyrs au Japon (✝ 1597)',
    paul_of_the_cross_priest: 'Saint Paul de la Croix, prêtre (✝ 1776)',
    paulinus_of_nola_bishop: 'Saint Paulin, évêque (✝ 431)',
    perpetua_of_carthage_and_felicity_of_carthage_martyrs:
      'Saintes Perpétue et Félicité, martyres à Carthage (✝ 203)',
    peter_canisius_priest: 'Saint Pierre Canisius, prêtre et docteur de l’Église (✝ 1597)',
    peter_chanel_priest: 'Saint Pierre Chanel, prêtre et martyr (✝ 1841)',
    peter_chrysologus_bishop:
      'Saint Pierre Chrysologue, évêque de Ravenne et docteur de l’Église (✝ 451)',
    peter_claver_priest: 'Saint Pierre Claver, prêtre Jésuite (✝ 1654)',
    peter_damian_bishop: 'Saint Pierre Damien, évêque d’Ostie, docteur de l’Église (✝ 1072)',
    peter_julian_eymard_priest:
      'Saint Pierre-Julien Eymard, prêtre, fondateur des Pères du Saint-Sacrement (✝ 1868)',
    philip_and_james_apostles: 'Saint Philippe et Saint Jacques le Mineur, apôtres',
    philip_neri_priest: 'Saint Philippe Néri, prêtre (✝ 1595)',
    pirmin_of_hornbach_abbot: 'Saint Pirmin, abbé (✝ 753)',
    pius_of_pietrelcina_priest: 'Saint Pio de Pietrelcina (Padre Pio), prêtre Capucin (✝ 1968)',
    pius_v_pope: 'Saint Pie V, pape (✝ 1572)',
    pius_x_pope: 'Saint Pie X, pape (✝ 1914)',
    polycarp_of_smyrna_bishop: 'Saint Polycarpe, évêque et martyre (✝ 167)',
    pontian_i_pope_and_hippolytus_of_rome_priest:
      'Saints Pontien, pape, et Hippolyte, prêtre de Rome, martyrs (✝ 235)',
    pothinus_of_lyon_bishop_blandina_of_lyon_virgin_and_companions_martyrs:
      'Saint Pothin, évêque, Sainte Blandine, vierge, et leurs 46 compagnons, martyrs à Lyon (✝ 177)',
    presentation_of_mary: 'Présentation de la vierge Marie',
    queenship_of_mary: 'Vierge Marie, reine',
    raymond_of_penyafort_priest: 'Saint Raymond de Peñafort, prêtre (✝ 1275)',
    remigius_of_reims_bishop: 'Saint Remi, évêque de Reims (✝ 530)',
    richardis_of_swabi: 'Sainte Richarde, impératrice (✝ 894 ou 896)',
    rita_of_cascia_religious: 'Sainte Rita da Cascia, veuve puis religieuse (✝ 1456)',
    robert_bellarmine_bishop:
      'Saint Robert Bellarmin, Jésuite, évêque et docteur de l’Église (✝ 1621)',
    romuald_of_ravenna_abbot: 'Saint Romuald, anachorète et père des moines Camaldules (✝ 1027)',
    rosalie_rendu_virgin: 'Bienheureuse Rosalie Rendu, vierge († 1856)',
    rose_of_lima_virgin: 'Sainte Rose de Lima, vierge (✝ 1617)',
    scholastica_of_nursia_virgin: 'Sainte Scholastique, Moniale, sœur de Saint Benoît (✝ 543)',
    sebastian_of_milan_martyr: 'Saint Sébastien, martyr (✝ v. 284)',
    seven_holy_founders_of_the_servite_order:
      'Saint Alexis Falconieri et les fondateurs des Servites (✝ 1310)',
    sharbel_makhluf_priest: 'Saint Charbel Makhlouf, moine prêtre Maronite (✝ 1898)',
    simon_and_jude_apostles: 'Saint Simon (le Zélote) et Saint Jude (Thaddée), apôtres',
    sixtus_ii_pope_and_companions_martyrs: 'Saint Sixte II, pape, et ses diacres, martyrs (✝ 258)',
    stanislaus_of_szczepanow_bishop: 'Saint Stanislas, évêque de Cracovie, martyr (✝ 1079)',
    stanislaus_of_szczepanow_bishop_patron_of_poland:
      'Saint Stanislas, évêque de Cracovie, martyr et patron de la Pologne (✝ 1079)',
    stephen_i_of_hungary: 'Saint Étienne, roi de Hongrie (✝ 1038)',
    stephen_the_first_martyr: 'Saint Étienne, diacre et premier martyr (✝ 35)',
    sylvester_i_pope: 'Saint Sylvestre Ier, pape (✝ 335)',
    teresa_benedicta_of_the_cross_stein_virgin:
      'Sainte Thérèse-Bénédicte de la Croix (Edith Stein), Carmélite, martyr en Pologne (✝ 1942)',
    teresa_benedicta_of_the_cross_stein_virgin_copatroness_of_europe:
      'Sainte Thérèse-Bénédicte de la Croix (Edith Stein), Carmélite, martyr en Pologne, co-patronne de l’Europe (✝ 1942)',
    teresa_of_jesus_of_avila_virgin:
      'Sainte Thérèse de Jésus (d’Avila), vierge et docteur de l’Église (✝ 1582)',
    therese_of_the_child_jesus_and_the_holy_face_of_lisieux_virgin:
      'Sainte Thérèse de l’Enfant-Jésus, vierge et docteur de l’Église (✝ 1897)',
    therese_of_the_child_jesus_and_the_holy_face_of_lisieux_virgin_copatroness_of_france:
      'Sainte Thérèse de l’Enfant-Jésus, vierge, docteur de l’Église, co-patronne de la France (✝ 1897)',
    thomas_apostle: 'Saint Thomas, apôtre',
    thomas_aquinas_priest: 'Saint Thomas d’Aquin, frère prêcheur, docteur de l’Église (✝ 1274)',
    thomas_becket_bishop: 'Saint Thomas Becket, évêque et martyr (✝ 1170)',
    timothy_of_ephesus_and_titus_of_crete_bishops:
      'Saints Timothée et Tite, évêques, disciples et compagnons de Saint Paul (Ier s.)',
    translation_of_the_relics_of_odile_of_alsace_abbess: 'Translation des Reliques de Sainte Odile',
    turibius_of_mogrovejo_bishop: 'Saint Alphonse Turibe de Mogrovejo, évêque de Lima (✝ 1606)',
    ulrich_of_augsburg_bishop: 'Saint Ulrich, évêque (✝ 973)',
    urban_i_pape: 'Saint Urbain Ier, pape et martyr (IIIème s.)',
    valerius_of_treves_bishop: 'Saint Valère, évêque (IVème s.)',
    vincent_de_paul_priest:
      'Saint Vincent de Paul, prêtre, fondateur de la congrégation de la Mission et des Filles de la Charité (✝ 1660)',
    vincent_ferrer_priest: 'Saint Vincent Ferrier, prêtre de l’Ordre des Prêcheurs (✝ 1419)',
    vincent_of_saragossa_deacon: 'Saint Vincent, diacre et martyr (✝ 304)',
    visitation_of_mary: 'Visitation de la vierge Marie',
    wenceslaus_i_of_bohemia_martyr: 'Saint Venceslas, martyr (✝ 929)',
    wenceslaus_i_of_bohemia_martyr_patron_of_the_czech_nation:
      'Saint Venceslas, martyr et patron de la nation tchèque (✝ 929)',
    wendelin_of_trier_abbot: 'Saint Wendelin, ermite et abbé',
  },
};
