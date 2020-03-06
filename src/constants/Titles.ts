export type TTitleKeys =
  | 'PATRON_OF_EUROPE'
  | 'FEAST_OF_THE_LORD'
  | 'DOCTOR_OF_THE_CHURCH'
  | 'MARIAN_FEAST'
  | 'TRIDUUM'
  | 'MARTYR';

export type TTitles = {
  [key in TTitleKeys]: string;
};

const Titles: TTitles = {
  PATRON_OF_EUROPE: 'PATRON_OF_EUROPE',
  FEAST_OF_THE_LORD: 'FEAST_OF_THE_LORD',
  DOCTOR_OF_THE_CHURCH: 'DOCTOR_OF_THE_CHURCH',
  MARIAN_FEAST: 'MARIAN_FEAST',
  TRIDUUM: 'TRIDUUM',
  MARTYR: 'MARTYR',
};

export default Titles;
