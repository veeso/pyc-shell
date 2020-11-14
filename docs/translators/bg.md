# Bulgarian Transliteration

- [Bulgarian Transliteration](#bulgarian-transliteration)
  - [Cyrillic to latin](#cyrillic-to-latin)
  - [Latin to Cyrillic](#latin-to-cyrillic)

🇧🇬 This document contains the documentation for the rules used to transliterate Bulgarian Cyrillic 🇧🇬

## Cyrillic to latin

The conversion from cyrillic to latin follows the [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) standard with some differences. The entire conversion table is illustrated here below:

| Bulgarian | Latin | Notes                                                                                                                                                  |
|-----------|-------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| А         | A     |                                                                                                                                                        |
| Б         | B     |                                                                                                                                                        |
| К         | C     | K is translated into C, only when not followed ```'Е','Э','И','Й','Ы','ъ'```, or it is preceeded by ```'К','А','И','О'```. You can force a 'C' using ```'Кь'```  |
| Ч         | CH    |                                                                                                                                                        |
| Ц         | Z     |                                                                                                                                                        |
| Д         | D     |                                                                                                                                                        |
| Э         | E     |                                                                                                                                                        |
| Ф         | F     |                                                                                                                                                        |
| Г         | G     |                                                                                                                                                        |
| Х         | H     |                                                                                                                                                        |
| И         | I     |                                                                                                                                                        |
| Ж         | J     |                                                                                                                                                        |
| Й         | J     |                                                                                                                                                        |
| Ё         | JO    |                                                                                                                                                        |
| К         | K     | K is converted to latin K only when followed by ```'Е','Э','И','Й','Ы','ъ'``` ,or it is NOT preceeded by ```'К','А','И','О'``` .You can force a K using ```'КЪ'``` |
| Л         | L     |                                                                                                                                                        |
| М         | M     |                                                                                                                                                        |
| Н         | N     |                                                                                                                                                        |
| О         | O     |                                                                                                                                                        |
| П         | P     |                                                                                                                                                        |
| Кю        | Q     |                                                                                                                                                        |
| Р         | R     |                                                                                                                                                        |
| С         | S     |                                                                                                                                                        |
| Ш         | SH    |                                                                                                                                                        |
| Щ         | SHT   |                                                                                                                                                        |
| Т         | T     |                                                                                                                                                        |
| У         | U     |                                                                                                                                                        |
| В         | V     |                                                                                                                                                        |
| Вь        | W     |                                                                                                                                                        |
| КС        | X     |                                                                                                                                                        |
| Ы         | Y     |                                                                                                                                                        |
| Я         | YA    |                                                                                                                                                        |
| Е         | E    |                                                                                                                                                        |
| Ю         | YU    |                                                                                                                                                        |
| З         | Z     |                                                                                                                                                        |
| €         | $     |                                                                                                                                                        |
| Ъ         | '     |                                                                                                                                                        |
| Ь         | `     |                                                                                                                                                        |
| №         | #     |                                                                                                                                                        |

## Latin to Cyrillic

| Latin | Russian | Notes                         |
|-------|---------|-------------------------------|
| А     | A       |                               |
| B     | Б       |                               |
| C     | К       | Unless if followed by H       |
| CH    | Ч       |                               |
| Ч     | CH      |                               |
| D     | Д       |                               |
| E     | Е       |                               |
| F     | Ф       |                               |
| G     | Г       |                               |
| G     | ДЖ      | If g is followed by Y, E, I   |
| H     | Х       |                               |
| I     | И       | Unless if followed be U, A, O |
| IU    | Ю       |                               |
| IA    | Я       |                               |
| IO    | Ё       |                               |
| J     | Ж       |                               |
| K     | К       |                               |
| L     | Л       |                               |
| M     | М       |                               |
| N     | Н       |                               |
| O     | О       |                               |
| P     | П       |                               |
| Q     | КЮ      |                               |
| R     | Р       |                               |
| S     | С       | Unless if followed by H       |
| Sh    | Ш       |                               |
| T     | Т       |                               |
| TS    | Ц       | Unless if followed by S       |
| U     | У       |                               |
| V     | В       |                               |
| W     | У       |                               |
| X     | КС      |                               |
| Y     | Ы       | Unless if followed by E       |
| YE    | E       |                               |
| Z     | З       |                               |
