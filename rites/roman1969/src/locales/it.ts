import { Locale } from '../types/locale';

export const locale: Locale = {
  id: 'it',

  seasons: {
    advent: {
      season: 'Tempo du Avvento',
      weekday: '$t(weekdays:{{dow}}, capitalize) della {{week, romanize}} settimana di Avvento',
      sunday: '{{week, romanize}} domenica di Avvento',
      privileged_weekday: '{{day}} $t(months:11)',
    },

    christmas_time: {
      season: 'Tempo di Natale',
      day: '$t(weekdays:{{dow}}, capitalize) di Natale',
      octave: '{{count}}º giorno fra l’Ottava di Natale',
      second_sunday_after_christmas: 'II domenica dopo Natale',
      before_epiphany: '{{day}} $t(months:0)',
      after_epiphany: '$t(weekdays:{{dow}}, capitalize) dopo l’Epifania',
    },

    ordinary_time: {
      season: 'Tempo Ordinario',
      weekday: '$t(weekdays:{{dow}}, capitalize) della {{week, romanize}} settimana del Tempo Ordinario',
      sunday: '{{week, romanize}} domenica del Tempo Ordinario',
    },

    lent: {
      season: 'Tempo di Quaresima',
      weekday: '$t(weekdays:{{dow}}, capitalize) della {{week, romanize}} settimana di Quaresima',
      sunday: '{{week, romanize}} domenica di Quaresima',
      day_after_ash_wed: '$t(weekdays:{{dow}}, capitalize) dopo le Ceneri',
      holy_week_day: '$t(weekdays:{{dow}}, capitalize) della Settimana Santa',
    },

    paschal_triduum: {
      season: 'Triduo Pasquale',
    },

    easter_time: {
      season: 'Tempo di Pasqua',
      weekday: '$t(weekdays:{{dow}}, capitalize) della {{week, romanize}} settimana di Pasqua',
      sunday: '{{week, romanize}} domenica di Pasqua',
      octave: '$t(weekdays:{{dow}}, capitalize) fra l’Ottava di Pasqua',
    },
  },

  periods: {
    epiphany: 'Epifania',
    holy_week: 'Settimana Santa',
  },

  ranks: {
    solemnity: 'solennità',
    sunday: 'domenica',
    feast: 'festa',
    memorial: 'memoria',
    optional_memorial: 'memoria facoltativa',
    weekday: 'feria',
  },

  cycles: {
    proper_of_time: 'Proprio del Tempo',
    proper_of_saints: 'Proprio dei Santi',
    sunday_year_a: 'Anno A',
    sunday_year_b: 'Anno B',
    sunday_year_c: 'Anno C',
    weekday_year_1: 'Anno dispari',
    weekday_year_2: 'Anno pari',
    psalter_week_1: 'I Settimana del Salterio',
    psalter_week_2: 'II Settimana del Salterio',
    psalter_week_3: 'III Settimana del Salterio',
    psalter_week_4: 'IV Settimana del Salterio',
  },

  weekdays: {
    0: 'domenica',
    1: 'lunedì',
    2: 'martedì',
    3: 'mercoledì',
    4: 'giovedì',
    5: 'venerdì',
    6: 'sabato',
  },

  months: {
    0: 'gennaio',
    1: 'febbraio',
    2: 'marzo',
    3: 'aprile',
    4: 'maggio',
    5: 'giugno',
    6: 'luglio',
    7: 'agosto',
    8: 'settembre',
    9: 'ottobre',
    10: 'novembre',
    11: 'dicembre',
  },

  colors: {
    black: 'nero',
    gold: 'oro',
    green: 'verde',
    purple: 'viola',
    red: 'rosso',
    rose: 'rosa',
    white: 'bianca',
  },

  ordinals: {
    '1': 'primo',
    '1_feminine': 'prima',
    '2': 'secondo',
    '2_feminine': 'seconda',
    '3': 'terzo',
    '3_feminine': 'terza',
    '4': 'quarto',
    '4_feminine': 'quarta',
    '5': 'quinto',
    '5_feminine': 'quinta',
    '6': 'sesto',
    '6_feminine': 'sesta',
    '7': 'settimo',
    '7_feminine': 'settima',
    '8': 'ottavo',
    '8_feminine': 'ottava',
    '9': 'nono',
    '9_feminine': 'nona',
    '10': 'decimo',
    '10_feminine': 'decima',
    '11': 'undicesimo',
    '11_feminine': 'undicesima',
    '12': 'dodicesimo',
    '12_feminine': 'dodicesima',
    '13': 'tredicesimo',
    '13_feminine': 'tredicesima',
    '14': 'quattordicesimo',
    '14_feminine': 'quattordicesima',
    '15': 'quindicesimo',
    '15_feminine': 'quindicesima',
    '16': 'sedicesimo',
    '16_feminine': 'sedicesima',
    '17': 'diciassettesimo',
    '17_feminine': 'diciassettesima',
    '18': 'diciottesimo',
    '18_feminine': 'diciottesima',
    '19': 'diciannoveesimo',
    '19_feminine': 'diciannoveesima',
    '20': 'ventesimo',
    '20_feminine': 'ventesima',
    '21': 'ventunesimo',
    '21_feminine': 'ventunesima',
    '22': 'ventiduesimo',
    '22_feminine': 'ventiduesima',
    '23': 'ventitreesimo',
    '23_feminine': 'ventitreesima',
    '24': 'ventiquattresimo',
    '24_feminine': 'ventiquattresima',
    '25': 'venticinquesimo',
    '25_feminine': 'venticinquesima',
    '26': 'ventiseiesimo',
    '26_feminine': 'ventiseiesima',
    '27': 'ventisettesimo',
    '27_feminine': 'ventisettesima',
    '28': 'ventottesimo',
    '28_feminine': 'ventottesima',
    '29': 'ventinovesimo',
    '29_feminine': 'ventinovesima',
    '30': 'trentesimo',
    '30_feminine': 'trentesima',
    '31': 'trentuno',
    '31_feminine': 'trentuna',
    '32': 'trentaduesimo',
    '32_feminine': 'trentaduesima',
    '33': 'trentatreesimo',
    '33_feminine': 'trentatreesima',
    '34': 'trentaquattresimo',
    '34_feminine': 'trentaquattresima',
  },

  names: {
    adalbert_of_prague_bishop: 'Sant’Adalberto, vescovo e martire',
    adalbert_of_prague_bishop_patron_of_poland: 'Sant’Adalberto, vescovo, martire e patrono della Polonia',
    adelaide_of_burgundy_empress: 'Santa Adelaide di Borgogna, imperatrice',
    adelphus_of_metz_bishop: 'Sant’Adelfo di Metz, vescovo',
    agatha_of_sicily_virgin: 'Sant’Agata, vergine e martire',
    agnes_of_rome_virgin: 'Sant’Agnese, vergine e martire',
    albert_the_great_bishop: 'Sant’Alberto Magno, vescovo e dottore della Chiesa',
    all_saints: 'Tutti i Santi',
    all_saints_of_the_archdiocese_of_paris: 'Tutti i Santi dell’Arcidiocesi di Parigi',
    all_saints_of_the_diocese_of_saint_denis: 'Tutti i Santi della Diocesi di Saint-Denis',
    aloysius_gonzaga_religious: 'San Luigi Gonzaga, religioso',
    alphonsus_mary_liguori_bishop: 'Sant’Alfonso Maria de Liguori, vescovo e dottore della Chiesa',
    amandus_of_strasbourg_bishop: 'Sant’Amando di Strasburgo, vescovo',
    amarin_of_alsace_abbot: 'Sant’Amarino, abate e martire',
    ambrose_of_milan_bishop: 'Sant’Ambrogio, vescovo e dottore della Chiesa',
    andrew_apostle: 'Sant’Andrea, apostolo',
    andrew_apostle_patron_of_scotland: 'Sant’Andrea, apostolo e patrono della Scozia',
    andrew_dung_lac_priest_and_companions_martyrs: 'Santi Andrea Dung-Lac, sacerdote e compagni, martiri',
    andrew_kim_tae_gon_priest_paul_chong_ha_sang_and_companions_martyrs:
      'Sant Andrea Kim Taegon, sacerdote, Paolo Chong Hasang e compagni, martiri',
    angela_merici_virgin: 'Sant’Angela Merici, vergine',
    annunciation_of_the_lord: 'Annunciazione del Signore',
    anselm_of_canterbury_bishop: 'Sant’Anselmo, vescovo e dottore della Chiesa',
    ansgar_of_hamburg_bishop: 'Sant’Ansgario, vescovo',
    anthony_mary_claret_bishop: 'Sant’Antonio Marla Claret, vescovo',
    anthony_of_egypt_abbot: 'Sant’Antonio, abate',
    anthony_of_padua_priest: 'Sant’Antonio di Padova, sacerdote e dottore della Chiesa',
    anthony_zaccaria_priest: 'Sant’Antonio Maria Zaccaria, sacerdote',
    apollinaris_of_ravenna_bishop: 'Sant’Apollinare, vescovo e martire',
    arbogast_of_strasbourg_bishop: 'Sant’Arbogasto, vescovo',
    arbogast_of_strasbourg_bishop_patron_of_the_archdiocese_of_strasbourg:
      'Sant’Arbogasto, vescovo e patrono dell’Arcidiocesi di Strasburgo',
    ascension_of_the_lord: 'Ascensione del Signore',
    ash_wednesday: 'Mercoledì delle Ceneri',
    assumption_of_the_blessed_virgin_mary: 'Assunzione della Beata Vergine Maria',
    athanasius_of_alexandria_bishop: 'Sant’Atanasio, vescovo e dottore della Chiesa',
    audoen_of_rouen_bishop: 'Sant’Audoeno, vescovo',
    augustine_of_canterbury_bishop: 'Sant’Agostino di Canterbury, vescovo',
    augustine_of_hippo_bishop: 'Sant’Agostino, vescovo e dottore della Chiesa',
    augustine_zhao_rong_priest_and_companions_martyrs: 'Santi Agostino Zhao Rong, sacerdote, e compagni, martiri',
    aurelia_of_strasbourg_virgin: 'Sant’Aurèlia di Strasburgo, vergine',
    baptism_of_the_lord: 'Battesimo del Signore',
    barnabas_apostle: 'San Barnaba, apostolo',
    bartholomew_apostle: 'San Bartolomeo, apostolo',
    basil_the_great_and_gregory_nazianzen_bishops:
      'Santi Basilio Magno e Gregorio Nazianzeno, vescovi e dottori della Chiesa',
    bede_the_venerable_priest: 'San Beda Venerabile, sacerdote e dottore della Chiesa',
    benedict_of_aniane_abbot: 'San Benedetto d’Aniane, abate',
    benedict_of_nursia_abbot: 'San Benedetto, abate',
    benedict_of_nursia_abbot_patron_of_europe: 'San Benedetto, abate e patrono d’Europa',
    bernard_of_clairvaux_abbot: 'San Bernardo, abate e dottore della Chiesa',
    bernardine_of_siena_priest: 'San Bernardino da Siena, sacerdote',
    blaise_of_sebaste_bishop: 'San Biagio, vescovo e martire',
    blessed_martyrs_of_paris: 'Beati Martiri della Rivoluzione francese',
    bonaventure_of_bagnoregio_bishop: 'San Bonaventura, vescovo e dottore della Chiesa',
    boniface_of_mainz_bishop: 'San Bonifacio, vescovo e martire',
    bridget_of_sweden_religious: 'Santa Brigida, religiosa',
    bridget_of_sweden_religious_copatroness_of_europe: 'Santa Brigida, religiosa e patrona secondaria d’Europa',
    brigid_of_kildare_virgin: 'Santa Brigida di Kildare, badessa',
    brigid_of_kildare_virgin_copatroness_of_ireland: 'Santa Brigida di Kildare, badessa e patrona secondaria d’Irlanda',
    bruno_of_cologne_priest: 'San Bruno, sacerdote',
    cajetan_of_thiene_priest: 'San Gaetano, sacerdote',
    callistus_i_pope: 'San Callisto I, papa e martire',
    camillus_de_lellis_priest: 'San Camillo de Lellis, sacerdote',
    carmelites_of_compiegne_virgins_and_martyrs: 'Beati carmelitani di Compiègne, vergini e martiri',
    casimir_of_poland: 'San Casimiro',
    catherine_of_alexandria_virgin: 'Santa Caterina di Alessandria, vergine e martire',
    catherine_of_siena_virgin: 'Santa Caterina da Siena, vergine e dottore della Chiesa',
    catherine_of_siena_virgin_copatroness_of_europe:
      'Santa Caterina da Siena, vergine, dottore della Chiesa e patrona secondaria d’Europa',
    catherine_of_siena_virgin_copatroness_of_italy_and_europe:
      'Santa Caterina da Siena, vergine, dottore della Chiesa e patrona secondaria d’Italia e d’Europa',
    catherine_zoe_laboure_virgin: 'Santa Caterina Labourè, vergine',
    cecilia_of_rome_virgin: 'Santa Cecilia, vergine e martire',
    ceraunus_of_paris_bishop: 'San Cerauno di Parigi, vescovo',
    chair_of_saint_peter_the_apostle: 'Cattedra di San Pietro apostolo',
    charles_borromeo_bishop: 'San Carlo Borromeo, vescovo',
    charles_lwanga_and_companions_martyrs: 'San Carlo Lwanga e compagni, martiri',
    charles_of_jesus_de_foucauld: 'San Charles de Foucauld, sacerdote',
    christopher_magallanes_priest_and_companions_martyrs: 'Santi Cristoforo Magallanes, sacerdote, e compagni, martiri',
    clare_of_assisi_virgin: 'Santa Chiara, vergine',
    clement_i_pope: 'San Clemente I, papa e martire',
    columba_of_iona_abbot: 'San Colombano, abate',
    columba_of_iona_abbot_copatron_of_ireland: 'San Colombano, abate e patrono secondario dell’Irlanda',
    columban_of_luxeuil_abbot: 'San Colombano, abate', // src: mr_it_2020_ed3
    commemoration_of_all_the_faithful_departed: 'Commemorazione di tutti i fedeli defunti',
    conversion_of_saint_paul_the_apostle: 'Conversione di San Paolo apostolo',
    cornelius_i_pope_and_cyprian_of_carthage_bishop_martyrs: 'Santi Cornelio, papa e Cipriano, vescovo, martiri',
    cosmas_of_cilicia_and_damian_of_cilicia_martyrs: 'Santi Cosma e Damiano, martiri',
    cyril_constantine_the_philosopher_monk_and_methodius_michael_of_thessaloniki_bishop:
      'Santi Cirillo, monaco e Metodio, vescovo',
    cyril_constantine_the_philosopher_monk_and_methodius_michael_of_thessaloniki_bishop_copatrons_of_europe:
      'Santi Cirillo, monaco e Metodio, vescovo, patroni secondari d’Europa',
    cyril_of_alexandria_bishop: 'San Cirillo di Alessandria, vescovo e dottore della Chiesa',
    cyril_of_jerusalem_bishop: 'San Cirillo di Gerusalemme, vescovo e dottore della Chiesa',
    damasus_i_pope: 'San Damaso I, papa',
    daniel_brottier_priest: 'Beato Daniel Brottier, sacerdote',
    dedication_of_consecrated_churches:
      'Dedicazione di chiese consacrate di cui non si conosce la data di consacrazione',
    dedication_of_the_basilica_of_saint_mary_major: 'Dedicazione della Basilica di Santa Maria Maggiore',
    dedication_of_the_basilicas_of_saints_peter_and_paul_apostles:
      'Dedicazione delle Basiliche dei Santi Pietro e Paolo apostoli',
    dedication_of_the_cathedral_basilica_of_saint_denis_france:
      'Dedicazione della Basilica Cattedrale di Saint-Denis, Francia',
    dedication_of_the_cathedral_of_notre_dame_de_strasbourg_france:
      'Dedicazione della Cattedrale di Notre-Dame de Strasbourg, Francia',
    dedication_of_the_lateran_basilica: 'Dedicazione della basilica Lateranense',
    dedication_of_the_notre_dame_de_paris_cathedral_paris_france:
      'Dedicazione della Cattedrale di Notre-Dame de Paris, Francia',
    denis_of_paris_bishop_and_companions_martyrs: 'San Dionigi, vescovo e compagni, martiri',
    denis_of_paris_bishop_patron_of_the_archdiocese_of_paris:
      'San Dionigi, vescovo e martire, patrono dell’Arcidiocesi di Parigi',
    denis_of_paris_bishop_patron_of_the_city_and_of_the_diocese_of_saint_denis:
      'San Dionigi, vescovo e martire, patrono della città e della Diocesi di Saint-Denis',
    divine_mercy_sunday: 'II domenica di Pasqua o Domenica della Divina Misericordia',
    dominic_de_guzman_priest: 'San Domenico, sacerdote',
    easter_sunday: 'Domenica di Pasqua / Risurrezione del Signore',
    eligius_of_noyon_bishop: 'Sant’Eligio di Noyon, vescovo',
    elizabeth_of_hungary_religious: 'Sant’Elisabetta di Ungheria, religiosa',
    elizabeth_of_portugal: 'Santa Elisabetta di Portogallo',
    ephrem_the_syrian_deacon: 'Sant’Efrem, diacono e dottore della Chiesa',
    epiphany_of_the_lord: 'Epifania del Signore',
    eucharius_of_trier_bishop: 'Sant’Eucario, vescovo',
    eugenia_of_alsace_and_attala_of_alsace_virgins: 'Sante Eugenia e Attala, vergini',
    eusebius_of_vercelli_bishop: 'Sant’Eusebio di Vercelli, vescovo',
    exaltation_of_the_holy_cross: 'Esaltazione della Santa Croce',
    fabian_i_pope: 'San Fabiano, papa e martire',
    faustina_kowalska_virgin: 'Santa Faustina Kowalska, vergine', // src: https://it.wikipedia.org/wiki/Maria_Faustina_Kowalska
    fidelis_of_sigmaringen_priest: 'San Fedele da Sigmaringen, sacerdote e martire',
    first_martyrs_of_the_holy_roman_church: 'Santi Primi Martiri della Chiesa Romana',
    florentius_of_strasbourg_bishop: 'San Fiorenzo, vescovo',
    frances_of_rome_religious: 'Santa Francesca Romana, religiosa',
    francis_de_sales_bishop: 'San Francesco di Sales, vescovo e dottore della Chiesa',
    francis_of_assisi: 'San Francesco d’Assisi',
    francis_of_assisi_patron_of_italy: 'San Francesco d’Assisi, patrono d’Italia',
    francis_of_paola_hermit: 'San Francesco da Paola, eremita',
    francis_xavier_priest: 'San Francesco Saverio, sacerdote',
    frederic_ozanam_founder: 'Beato Federico Ozanam, fondatore',
    friday_of_the_passion_of_the_lord: 'Venerdì Santo / Passione del Signore',
    george_of_lydda_martyr: 'San Giorgio, martire',
    george_of_lydda_martyr_patron_of_england: 'San Giorgio, martire e patrono d’Inghilterra',
    germain_of_paris_bishop: 'San Germano di Parigi, vescovo',
    gertrude_the_great_virgin: 'Santa Geltrude, vergine',
    gregory_i_the_great_pope: 'San Gregorio Magno, papa e dottore della Chiesa',
    gregory_of_narek_abbot: 'San Gregorio di Narek, abate e dottore della Chiesa',
    gregory_vii_pope: 'San Gregorio VII, papa',
    hedwig_of_silesia_religious: 'Santa Edvige, religiosa',
    henry_ii_emperor: 'Sant’Enrico',
    henry_of_finland_bishop: 'Sant’Enrico, vescovo e martire',
    hilary_of_poitiers_bishop: 'Sant’Ilario, vescovo e dottore della Chiesa',
    hildegard_of_bingen_abbess: 'Ildegarda di Bingen, badessa e dottore della Chiesa', // src: https://it.wikipedia.org/wiki/Ildegarda_di_Bingen
    holy_family_of_jesus_mary_and_joseph: 'Santa Famiglia di Gesú, Maria e Giuseppe',
    holy_guardian_angels: 'Santi Angeli Custodi',
    holy_innocents_martyrs: 'Santi Innocenti, martiri',
    holy_saturday: 'Sabato Santo / Vigilia di Pasqua',
    holy_thursday: 'Giovedì Santo',
    ignatius_of_antioch_bishop: 'Sant’Ignazio di Antiochia, vescovo e martire',
    ignatius_of_loyola_priest: 'Sant’Ignazio di Loyola, sacerdote',
    immaculate_conception_of_the_blessed_virgin_mary: 'Immacolata Concezione della Beata Vergine Maria',
    immaculate_heart_of_mary: 'Cuore Immacolato della Beata Vergine Maria',
    innocent_v_pope: 'Beato Innocenzo V, Papa',
    irenaeus_of_lyon_bishop: 'Sant’Ireneo, vescovo, martire e dottore della Chiesa',
    isabelle_of_france_virgin: 'Beata Isabella di Francia, vergine',
    isidore_of_seville_bishop: 'Sant’Isidoro, vescovo e dottore della Chiesa',
    james_apostle: 'San Giacomo, apostolo',
    jane_frances_de_chantal_religious: 'Santa Giovanna Francesca de Chantal, religiosa',
    januarius_i_of_benevento_bishop: 'San Gennaro, vescovo e martire',
    jerome_emiliani: 'San Girolamo Emiliani',
    jerome_of_stridon_priest: 'San Girolamo, sacerdote e dottore della Chiesa',
    joachim_and_anne_parents_of_mary: 'Santi Gioacchino e Anna, genitori della Beata Vergine Maria',
    john_apostle: 'San Giovanni, apostolo ed evangelista',
    john_baptist_de_la_salle_priest: 'San Giovanni Battista de La Salle, sacerdote',
    john_bosco_priest: 'San Giovanni Bosco, sacerdote',
    john_chrysostom_bishop: 'San Giovanni Crisostomo, vescovo e dottore della Chiesa',
    john_damascene_priest: 'San Giovanni Damasceno, sacerdote e dottore della Chiesa',
    john_de_brebeuf_isaac_jogues_priests_and_companions_martyrs:
      'Santi Giovanni de Brébeuf e Isacco Jogues, sacerdoti e compagni, martiri',
    john_eudes_priest: 'San Giovanni Eudes, sacerdote',
    john_fisher_bishop_and_thomas_more_martyrs: 'Santi Giovanni Fisher, vescovo e Tommaso More, martiri',
    john_i_pope: 'San Giovanni I, papa e martire',
    john_leonardi_priest: 'San Giovanni Leonardi, sacerdote',
    john_mary_vianney_priest: 'San Giovanni Maria Vianney, sacerdote',
    john_of_avila_priest: 'San Giovanni d’Avila, sacerdote e dottore della Chiesa', // src: https://it.wikipedia.org/wiki/Giovanni_d%27Avila
    john_of_capistrano_priest: 'San Giovanni da Capestrano, sacerdote',
    john_of_god_duarte_cidade_religious: 'San Giovanni di Dio, religioso',
    john_of_kanty_priest: 'San Giovanni da Keti, sacerdote',
    john_of_the_cross_priest: 'San Giovanni della Croce, sacerdote e dottore della Chiesa',
    john_paul_ii_pope: 'San Giovanni Paolo II, papa',
    john_xxiii_pope: 'San Giovanni XXIII, papa',
    josaphat_kuntsevych_bishop: 'San Giosafat, vescovo e martire',
    joseph_of_calasanz_priest: 'San Giuseppe Calasanzio, sacerdote',
    joseph_spouse_of_mary: 'San Giuseppe, sposo della Beata Vergine Maria',
    joseph_the_worker: 'San Giuseppe lavoratore',
    josephine_bakhita_virgin: 'Santa Giuseppina Bakhita, vergine',
    juan_diego_cuauhtlatoatzin: 'San Giovanni Diego Cuauhtlatoatzin',
    justin_martyr: 'San Giustino, martire',
    landry_of_paris_bishop: 'San Landerico di Parigi, vescovo',
    lawrence_of_brindisi_priest: 'San Lorenzo da Brindisi, sacerdote e dottore della Chiesa',
    lawrence_of_rome_deacon: 'San Lorenzo, diacono e martire',
    lawrence_ruiz_and_companions_martyrs: 'Santi Lorenzo Ruiz e compagni, martiri',
    leo_i_the_great_pope: 'San Leone Magno, papa e dottore della Chiesa',
    leodegar_of_autun_bishop: 'San Leodegario di Autun, vescovo e martire',
    louis_grignion_de_montfort_priest: 'San Luigi Maria Grignion de Montfort, sacerdote',
    louis_ix_of_france: 'San Ludovico',
    louise_de_marillac_religious: 'Santa Luisa di Marillac, religiosa',
    lucy_of_syracuse_virgin: 'Santa Lucia, vergine e martire',
    ludan_of_scotland_pilgrim: 'San Ludano, pellegrino',
    luke_evangelist: 'San Luca, evangelista',
    madeleine_sophie_barat_virgin: 'Santa Maddalena Sofia Barat, vergine',
    marcellinus_of_rome_and_peter_the_exorcist_martyrs: 'Santi Marcellino e Pietro, martiri',
    marcellus_of_paris_bishop: 'San Marcello di Parigi, vescovo',
    margaret_mary_alacoque_virgin: 'Santa Margherita Maria Alacoque, vergine',
    margaret_of_scotland: 'Santa Margherita di Scozia',
    maria_goretti_virgin: 'Santa Maria Goretti, vergine e martire',
    marie_eugenie_of_jesus_milleret_de_brou_virgin:
      'Santa Maria Eugenia di Gesù Milleret de Brou, vergine e fondatrice',
    mark_evangelist: 'San Marco, evangelista',
    martha_of_bethany_mary_of_bethany_and_lazarus_of_bethany: 'Santa Marta, Maria e Lazzaro',
    martin_de_porres_religious: 'San Martino di Porres, religioso',
    martin_i_pope: 'San Martino I, papa e martire',
    martin_of_tours_bishop: 'San Martino di Tours, vescovo',
    mary_magdalene: 'Santa Maria Maddalena',
    mary_magdalene_de_pazzi_virgin: 'Santa Maria Maddalena de’ Pazzi, vergine',
    mary_mother_of_god: 'Maria santissima Madre di Dio',
    mary_mother_of_the_church: 'Beata Vergine Maria Madre della Chiesa',
    mary_of_the_incarnation_barbara_acarie_religious: 'Beata Maria dell’Incarnazione Barbara Acarie, religiosa',
    mary_of_the_incarnation_marie_guyart_religious: 'Santa Maria dell’Incarnazione Maria Guyart, religiosa',
    mary_of_the_providence_eugenie_smet_virgin: 'Beata Maria della Provvidenza Eugènia Smet, vergine',
    mary_of_the_sacred_heart_sophie_therese_de_soubiran_la_louviere_virgin:
      'Beata Maria Teresa de Soubiran La Louvière, vergine',
    maternus_of_cologne_bishop: 'San Materno, vescovo',
    maternus_of_cologne_valerius_of_trier_and_eucharius_of_trier_bishops: 'Santi Materno, Valerio e Eucario, vescovi',
    matthew_apostle: 'San Matteo, apostolo ed evangelista',
    matthias_apostle: 'San Mattia, apostolo',
    maximilian_mary_raymund_kolbe_priest: 'San Massimiliano Maria Kolbe, sacerdote e martire',
    mederic_of_autun_and_droctoveus_of_autun_abbots: 'Santi Mederico e Drottoveo, abati',
    michael_gabriel_and_raphael_archangels: 'Santi Arcangeli Michele, Gabriele e Raffaele',
    modestus_andlauer_and_andrew_bauer_martyrs: 'San Modesto Andlauer e Andrea Bauer, martiri',
    modestus_andlauer_martyr: 'San Modesto Andlauer, martire',
    monica_of_hippo: 'Santa Monica',
    morand_of_cluny_monk: 'San Morando, monaco',
    most_holy_body_and_blood_of_christ: 'Santissimo Corpo e Sangue di Cristo',
    most_holy_name_of_jesus: 'Santissimo Nome di Gesú',
    most_holy_name_of_mary: 'Santissimo Nome di Maria',
    most_holy_trinity: 'Santissima Trinitá',
    most_sacred_heart_of_jesus: 'Sacratissimo Cuore di Gesù',
    nativity_of_john_the_baptist: 'Nativitá di San Giovanni Battista',
    nativity_of_the_blessed_virgin_mary: 'Nativitá della Beata Vergine Maria',
    nativity_of_the_lord: 'Natale del Signore',
    nereus_of_terracina_and_achilleus_of_terracina_martyrs: 'Santi Nereo e Achilleo, martiri',
    nicholas_barre_priest: 'Beato Nicola Barré, sacerdote',
    nicholas_of_myra_bishop: 'San Nicola, vescovo',
    norbert_of_xanten_bishop: 'San Norberto, vescovo',
    odile_of_alsace_abbess: 'Santa Odìlia, badessa',
    odile_of_alsace_abbess_patroness_of_alsace: 'Santa Odìlia, badessa, Patrona dell’Alsazia',
    our_lady_of_fatima: 'Beata Vergine Maria di Fatima',
    our_lady_of_guadalupe: 'Beata Vergine Maria di Guadalupe',
    our_lady_of_guadalupe_patroness_of_the_americas: 'Beata Vergine Maria di Guadalupe, Patrona delle Americhe',
    our_lady_of_loreto: 'Beata Vergine Maria di Loreto', // src: mr_it_2020_ed3
    our_lady_of_lourdes: 'Beata Vergine Maria di Lourdes',
    our_lady_of_mount_carmel: 'Beata Vergine Maria del Carmelo',
    our_lady_of_sorrows: 'Beata Vergine Maria Addolorata',
    our_lady_of_sorrows_patroness_of_slovakia: 'Beata Vergine Maria Addolorata, patrona della Slovacchia',
    our_lady_of_the_miraculous_medal: 'Beata Vergine Maria della Medaglia Miracolosa',
    our_lady_of_the_rosary: 'Beata Vergine Maria del Rosario',
    our_lady_refuge_of_sinners: 'Beata Vergine Maria, rifugio dei peccatori',
    our_lord_jesus_christ_king_of_the_universe: 'Cristo Re dell’universo',
    palm_sunday_of_the_passion_of_the_lord: 'Domenica delle Palme: Passione del Signore',
    pancras_of_rome_martyr: 'San Pancrazio, martire',
    passion_of_saint_john_the_baptist: 'Martirio di San Giovanni Battista',
    patrick_of_ireland_bishop: 'San Patrizio, vescovo',
    patrick_of_ireland_bishop_patron_of_ireland: 'San Patrizio, vescovo e patrono dell’Irlanda',
    paul_miki_and_companions_martyrs: 'San Paolo Miki sacerdote e compagni, martiri',
    paul_of_the_cross_priest: 'San Paolo della Croce, sacerdote',
    paul_vi_pope: 'San Paolo VI, papa', // src: mr_it_2020_ed3
    paulinus_of_nola_bishop: 'San Paolino di Nola, vescovo',
    pentecost_sunday: 'Domenica di Pentecoste',
    perpetua_of_carthage_and_felicity_of_carthage_martyrs: 'Sante Perpetua e Felicita, martiri',
    peter_and_paul_apostles: 'Santi Pietro e Paolo, apostoli',
    peter_canisius_priest: 'San Pietro Canisio, sacerdote e dottore della Chiesa',
    peter_chanel_priest: 'San Pietro Chanel, sacerdote e martire',
    peter_chrysologus_bishop: 'San Pietro Crisologo, vescovo e dottore della Chiesa',
    peter_claver_priest: 'San Pietro Claver, sacerdote',
    peter_damian_bishop: 'San Pier Damiani, vescovo e dottore della Chiesa',
    peter_julian_eymard_priest: 'San Pietro Giuliano Eymard, sacerdote',
    philip_and_james_apostles: 'Santi Filippo e Giacomo, apostoli',
    philip_neri_priest: 'San Filippo Neri, sacerdote',
    pius_francesco_forgione_priest: 'San Pio da Pietrelcina, sacerdote',
    pius_v_pope: 'San Pio V, papa',
    pius_x_pope: 'San Pio X, papa',
    polycarp_of_smyrna_bishop: 'San Policarpo, vescovo e martire',
    pontian_i_pope_and_hippolytus_of_rome_priest: 'Santi Ponziano, papa e Ippolito, sacerdote, martiri',
    presentation_of_the_blessed_virgin_mary: 'Presentazione della Beata Vergine Maria',
    presentation_of_the_lord: 'Presentazione del Signore',
    queenship_of_the_blessed_virgin_mary: 'Beata Vergine Maria regina',
    raymond_of_penyafort_priest: 'San Raimondo di Peñafort, sacerdote',
    richardis_of_swabia_empress: 'Santa Riccarda, imperatrice',
    rita_of_cascia_religious: 'Santa Rita da Cascia, religiosa',
    robert_bellarmine_bishop: 'San Roberto Bellarmino, vescovo e dottore della Chiesa',
    romuald_of_ravenna_abbot: 'San Romualdo, abate',
    rosalie_jeanne_marie_rendu_virgin: 'Beata Rosalia Rendu, vergine',
    rose_of_lima_virgin: 'Santa Rosa da Lima, vergine',
    scholastica_of_nursia_virgin: 'Santa Scolastica, vergine',
    sebastian_of_milan_martyr: 'San Sebastiano, martire',
    seven_holy_founders_of_the_servite_order: 'Santi Sette Fondatori dei Servi della Beata Vergine Maria',
    sharbel_makhluf_priest: 'San Charbel Makhluf, sacerdote',
    simon_and_jude_apostles: 'Santi Simone e Giuda, apostoli',
    sixtus_ii_pope_and_companions_martyrs: 'San Sisto II, papa e compagni, martiri',
    stanislaus_of_szczepanow_bishop: 'Santo Stanislao, vescovo e martire',
    stanislaus_of_szczepanow_bishop_patron_of_poland: 'Santo Stanislao, vescovo, martire e patrono della Polonia',
    stephen_i_of_hungary: 'Santo Stefano di Ungheria',
    stephen_the_first_martyr: 'Santo Stefano, primo martire',
    sunday_of_the_word_of_god: 'III domenica del Tempo Ordinario, o Domenica della Parola di Dio',
    sylvester_i_pope: 'San Silvestro I, papa',
    teresa_benedicta_of_the_cross_stein_virgin: 'Santa Teresa Benedetta della Croce, vergine e martire',
    teresa_benedicta_of_the_cross_stein_virgin_copatroness_of_europe:
      'Santa Teresa Benedetta della Croce, vergine, martire e patrona secondaria d’Europa',
    teresa_of_jesus_of_avila_virgin: 'Santa Teresa di Gesù d’Avila, vergine e dottore della Chiesa',
    therese_of_the_child_jesus_and_the_holy_face_of_lisieux_virgin:
      'Santa Teresa di Gesu’ Bambino, vergine e dottore della Chiesa',
    thomas_apostle: 'San Tommaso, apostolo',
    thomas_aquinas_priest: 'San Tommaso d’Aquino, sacerdote e dottore della Chiesa',
    thomas_becket_bishop: 'San Tommaso Becket, vescovo e martire',
    thomas_jean_georges_rehm_priest: 'Beato Tommaso Jean-Georges Rehm, sacerdote e martire',
    thursday_of_the_lords_supper: 'Giovedì Santo / Cena del Signore',
    timothy_of_ephesus_and_titus_of_crete_bishops: 'Santi Timoteo e Tito, vescovi',
    transfiguration_of_the_lord: 'Trasfigurazione del Signore',
    translation_of_the_relics_of_odile_of_alsace_abbess: 'Traslazione delle reliquie di santa Odìlia',
    turibius_of_mogrovejo_bishop: 'San Turibio de Mogrovejo, vescovo',
    urban_i_pope: 'Sant’Urbano I, papa',
    valerius_of_trier_bishop: 'San Valerio, vescovo',
    vincent_de_paul_priest: 'San Vincenzo de’ Paoli, sacerdote',
    vincent_ferrer_priest: 'San Vincenzo Ferrer, sacerdote',
    vincent_of_saragossa_deacon: 'San Vincenzo, diacono e martire',
    visitation_of_mary: 'Visitazione della Beata Vergine Maria',
    wenceslaus_i_of_bohemia_martyr: 'San Venceslao, martire',
    wenceslaus_i_of_bohemia_martyr_patron_of_the_czech_nation: 'San Venceslao, martire e patrono della nazione ceca',
    wendelin_of_trier_hermit: 'San Vendelino, eremita',
  },
};