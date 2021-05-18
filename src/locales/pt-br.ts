import { RomcalLocale } from '@romcal/models/locale/romcal-locale.type';

export default {
  advent: {
    season: 'Advento',
    weekday: '{{day}} da {{week}} semana do Advento',
    sunday: '{{week}} Domingo do Advento',
  },
  christmastide: {
    season: 'Natal',
    day: '{{day}} do Tempo do Natal',
    octave: '{{count}} dia da Oitava de Natal',
    sunday: '{{count}} Domingo do Natal',
  },
  epiphany: {
    season: 'Epifania do Senhor',
    before: '{{day}} antes da Epifania do Senhor',
    after: '{{day}} depois da Epifania do Senhor',
  },
  ordinary_time: {
    season: 'Tempo Comum',
    weekday: '{{day}} da {{week}} semana do Tempo Comum',
    sunday: '{{week}} Domingo do Tempo Comum',
  },
  lent: {
    season: 'Quaresma',
    weekday: '{{day}} da {{week}} semana da Quaresma',
    sunday: '{{week}} Domingo da Quaresma',
    day_after_ash_wed: '{{day}} depois da Quarta-feira de Cinzas',
  },
  holy_week: {
    season: 'Semana Santa',
    weekday: '{{day}} of Semana Santa',
  },
  paschal_triduum: {
    season: 'Tríduo Pascal',
  },
  eastertide: {
    season: 'Páscoa',
    weekday: '{{day}} da {{week}} semana do Tempo Pascal',
    sunday: '{{week}} Domingo do Tempo Pascal',
    octave: 'Tempo Pascal {{day}}',
  },
  liturgical_colors: {
    BLACK: 'preto',
    GOLD: 'dourado',
    GREEN: 'verde',
    PURPLE: 'roxo',
    RED: 'vermelho',
    ROSE: 'roséo',
    WHITE: 'branco',
  },
  ranks: {
    SOLEMNITY: 'solenidade',
    SUNDAY: 'domingo',
    TRIDUUM: 'Tríduo Pascal',
    HOLY_WEEK: 'Semana Santa',
    FEAST: 'festa',
    MEMORIAL: 'memória',
    OPT_MEMORIAL: 'memória facultativa',
    COMMEMORATION: 'comemoração',
    WEEKDAY: 'dia de semana',
  },
  celebrations: {
    all_saints: 'Todos os Santos',
    annunciation: 'Anunciação do Senhor',
    ascension: 'Ascensão do Senhor',
    ash_wednesday: 'Quarta-feira de Cinzas',
    assumption: 'Assunção da Bem Aventurada Virgem Maria',
    baptism_of_the_lord: 'Batismo do Senhor',
    christ_the_king_sunday: 'Cristo Rei do Universo',
    christmas: 'Natal',
    corpus_christi: 'Corpus Christi',
    divine_mercy_sunday: 'Domingo da Misericórdia',
    easter_sunday: 'Domingo da Páscoa',
    epiphany: 'Epifania do Senhor',
    exaltation_of_the_holy_cross: 'Exaltação da Santa Cruz',
    good_friday: 'Sexta-feira Santa',
    holy_family: 'Sagrada Família',
    holy_saturday: 'Sábado Santo',
    holy_thursday: 'Quinta-feira Santa',
    immaculate_conception_of_mary: 'Imaculada Conceição',
    immaculate_heart_of_mary: 'Imaculado Coração de Maria',
    joseph_spouse_of_mary: 'São José, esposo de Maria',
    mary_mother_of_god: 'Maria, Mãe de Deus',
    most_sacred_heart_of_jesus: 'Sagrado Coração de Jesus',
    nativity_of_john_the_baptist: 'Nascimento de João Batista',
    palm_sunday: 'Domingo de Ramos',
    pentecost_sunday: 'Pentecostes',
    peter_and_paul_apostles: 'São Pedro e São Paulo, Apóstolos',
    presentation_of_the_lord: 'Apresentação do Senhor',
    transfiguration: 'Transfiguração do Senhor',
    trinity_sunday: 'Santíssima Trindade',
  },
  sanctoral: {
    john_paul_ii_pope: 'Papa São João Paulo II',
    john_xxiii_pope: 'Papa São João XXIII',
    our_lady_help_of_christians: 'Nossa Senhora, Auxílio dos Cristãos',
    our_lady_mediatrix_of_all_grace: 'Nossa Senhora, Medianeira de todas as Graças',
    our_lady_mother_of_divine_providence_patroness_of_puerto_rico: 'Nossa Senhora, Mãe da Divina Providência', // TODO: Add `Patronness of Porto Rico` title
    our_lady_of_aparecida_patroness_of_brazil: 'Nossa Senhora Aparecida', // TODO: Add `Patronness of Brazil` title
    our_lady_of_china: 'Nossa Senhora da China',
    our_lady_of_fatima: 'Nossa Senhora de Fatima',
    our_lady_of_good_counsel: 'Nossa Senhora do Bom Conselho',
    our_lady_of_guadalupe: 'Nossa Senhora de Guadalupe',
    our_lady_of_hungary_patroness_of_hungary: 'Nossa Senhora da Hungria', // TODO: Translate `Out Lady of Hungary, Patroness of Hungary`
    our_lady_of_loreto: 'Nossa Senhora de Loreto',
    our_lady_of_lourdes: 'Nossa Senhora de Lourdes',
    our_lady_of_lujan_patroness_of_argentina: 'Nossa Senhora de Luján, padroeira da Argentina',
    our_lady_of_mount_carmel: 'Nossa Senhora do Monte Carmelo',
    our_lady_of_perpetual_help: 'Nossa Senhora do Perpétuo Socorro',
    our_lady_of_the_gate_of_dawn: 'Nossa Senhora da Porta da Aurora',
    our_lady_of_the_pillar: 'Nossa Senhora do Pilar',
    our_lady_of_the_rosary: 'Nossa Senhora do Rosário',
    our_lady_queen_of_peace: 'Nossa Senhora Rainha da Paz',
    presentation_of_mary: 'Apresentação da Virgem Maria',
    wenceslaus_i_of_bohemia_martyr: 'São Venceslau, mártir',
    wenceslaus_i_of_bohemia_martyr_patron_of_the_czech_nation: 'São Venceslau, mártir e patrono da nação tcheca',
  },
} as RomcalLocale;
