# Analyse des types Rust pour génération JSON Schema

## 1. Problèmes identifiés dans generate_schema.rs liés aux types

### 1.1 Fixes manuels pour des types non-idéaux

**Fichier:** `generate_schema.rs` lignes 79-134

Les types `DateDefExceptions` et `SaintCount` nécessitent des fixes manuels dans le générateur car leur schéma auto-généré n'est pas optimal.

**DateDefExceptions:**

- Type Rust: `#[serde(untagged)] enum` avec Single/Multiple
- Fix requis: Création manuelle d'un `anyOf` dans le JSON Schema
- Problème: Le fix accède fragilement à `CalendarDefinition.properties.date_exceptions`

**SaintCount:**

- Type Rust: Enum avec custom Serialize/Deserialize
- Fix requis: Remplacement du schéma auto-généré par un schéma manuel
- Problème: La génération automatique ne produit pas le schéma attendu

## 2. Analyse détaillée des types

### 2.1 Enums avec ordre important (Precedence)

**Fichier:** `core/src/types/liturgical/precedence.rs`

**État actuel:**

- Utilise `EnumIter` pour garantir l'ordre
- Tests explicites de l'ordre (lignes 181-242)
- Ordre critique pour la hiérarchie liturgique

**Évaluation:** ✅ Adéquat - L'ordre est correctement géré et testé

**Recommandation:** Maintenir tel quel. L'ordre est nécessaire pour la logique métier.

### 2.2 SingleOrMultiple Pattern (CommonsDef, ColorsDef)

**Fichier:** `core/src/types/calendar/day_definition.rs` lignes 14-27

**Pattern actuel:**

```rust
#[serde(untagged)]
pub enum CommonsDef {
    Single(CommonDefinition),
    Multiple(Vec<CommonDefinition>),
}
```

**Contraintes:**

- L'ordre est important pour CommonsDef et ColorsDef
- Permet flexibilité JSON: `"MARTYRS"` ou `["MARTYRS", "VIRGINS"]`

**Recommandation: Toujours normaliser en Multiple (array)**

**Rationale:**

1. Simplifie le schéma JSON (un seul type: array)
2. Élimine la complexité du untagged enum dans le schéma
3. L'ordre est préservé naturellement dans les arrays
4. Simplifie la désérialisation côté Rust
5. Plus prévisible pour les générateurs TypeScript

**Implémentation proposée:**

- Modifier la désérialisation pour accepter les deux formes mais normaliser en `Vec<T>`
- Le champ reste `Option<Vec<CommonDefinition>>` au lieu de l'enum
- Custom `Deserialize` qui accepte single value ou array, mais stocke toujours un array
- Type Rust plus simple = schéma JSON plus simple

**Exemple:**

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CommonsDef(pub Vec<CommonDefinition>);

impl<'de> Deserialize<'de> for CommonsDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        use serde::de;
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = CommonsDef;
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where A: de::SeqAccess<'de>
            {
                let mut vec = Vec::new();
                while let Some(elem) = seq.next_element()? {
                    vec.push(elem);
                }
                Ok(CommonsDef(vec))
            }
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where E: de::Error,
            {
                // Deserialize single value as array
                Ok(CommonsDef(vec![CommonDefinition::deserialize(
                    serde_json::Value::String(v.to_string())
                )?]))
            }
            // ... etc
        }
        deserializer.deserialize_any(Visitor)
    }
}
```

**Alternative plus simple si serde_json est déjà disponible:**

```rust
impl<'de> Deserialize<'de> for CommonsDef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        match value {
            serde_json::Value::Array(arr) => {
                Ok(CommonsDef(arr.into_iter()
                    .map(|v| CommonDefinition::deserialize(v))
                    .collect::<Result<Vec<_>, _>>()?))
            }
            _ => {
                // Single value -> wrap in array
                Ok(CommonsDef(vec![CommonDefinition::deserialize(value)?]))
            }
        }
    }
}
```

### 2.3 Untagged Enums - Cas complexes

**Types concernés:** `DateDef`, `DateDefExceptions`, `EntityPointer`, `SaintDateDef`, `DateDefExtended`, `ExceptionCondition`

**DateDefExceptions - Analyse:**

- Type Rust: `#[serde(untagged)] enum { Single, Multiple }`
- Fix manuel requis dans generate_schema.rs
- Même pattern que CommonsDef mais pour les exceptions de dates

**Recommandation:**

- Appliquer la même logique que CommonsDef: normaliser en `Vec<DateDefException>`
- Custom Deserialize qui accepte single object ou array
- Simplifie le schéma et élimine le fix manuel

**DateDef, ExceptionCondition, etc.:**

- Ces untagged enums représentent de vrais choix structuraux différents
- ✅ Maintenir tel quel - La complexité est justifiée par la sémantique

### 2.4 SaintCount - Custom serialization avec schema_with

**Fichier:** `core/src/types/entity/saint_count.rs`

**Recommandation confirmée: Option 1 - Ajouter schema_with personnalisé**

**Quand utiliser schema_with:**

1. Custom Serialize/Deserialize qui ne correspond pas au schéma auto-généré
2. Union de types différents (int | string | null)
3. Contraintes de validation complexes non capturées automatiquement
4. Transformations de format (ex: snake_case → SCREAMING_SNAKE_CASE)

**Implémentation pour SaintCount:**

```rust
#[cfg(feature = "schema-gen")]
fn saint_count_schema(_gen: &mut SchemaGenerator) -> Schema {
    serde_json::from_value(serde_json::json!({
        "anyOf": [
            {
                "type": "integer",
                "format": "uint32",
                "minimum": 0
            },
            {
                "const": "MANY",
                "type": "string"
            },
            {
                "type": "null"
            }
        ]
    })).unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "schema-gen", schemars(schema_with = "saint_count_schema"))]
pub enum SaintCount {
    Number(u32),
    Many(String),
}
```

**Avantage:** Élimine le fix manuel dans generate_schema.rs ligne 107-133

### 2.5 EntityPointer - Untagged enum

**Fichier:** `core/src/types/calendar/entity_pointer.rs`

**État actuel:**

```rust
#[serde(untagged)]
pub enum EntityPointer {
    ResourceId(String),
    Override(EntityOverride),
}
```

**Évaluation:** ✅ Adéquat

- Représente un vrai choix structurel (ID string vs object)
- Schéma généré correct
- Pas de fix manuel requis

**Recommandation:** Maintenir tel quel

### 2.6 Newtype wrappers (DayOfWeek, MonthIndex)

**Évaluation:** ✅ Parfaits - Pas de changement nécessaire

### 2.7 Types alias vs Newtype

**Recommandation:**

- **Type alias (`type EntityId = String`)**: Utiliser quand la sémantique est purement documentaire
- **Newtype wrapper**: Utiliser quand validation/contraintes sont nécessaires (ex: DayOfWeek)
- Pour JSON Schema: Les deux apparaissent comme `string`, mais newtype permet validation Rust

## 3. Plan d'action prioritaire

### Étape 1: Simplifier SingleOrMultiple → Multiple normalisé

1. Modifier `CommonsDef` pour accepter single/array mais stocker toujours Vec
2. Modifier `ColorsDef` de la même manière
3. Modifier `DateDefExceptions` pour accepter single/array mais stocker toujours Vec
4. Supprimer les fixes manuels correspondants dans generate_schema.rs

### Étape 2: Ajouter schema_with pour SaintCount

1. Créer `saint_count_schema()` function
2. Ajouter `#[schemars(schema_with = "saint_count_schema")]`
3. Supprimer le fix manuel SaintCountFix dans generate_schema.rs

### Étape 3: Documenter les patterns

1. Créer documentation quand utiliser:

   - Untagged enum (vrais choix structurels) vs normalisation array
   - schema_with (custom serialization) vs auto-génération
   - Newtype (validation) vs type alias (documentation)

## 4. Patterns à maintenir

✅ **Newtype wrappers** (DayOfWeek, MonthIndex): Excellents pour la sécurité

✅ **EnumIter pour ordre** (Precedence): Nécessaire pour la logique métier

✅ **Custom schema functions** (MassContent): Correct pour transformations complexes

✅ **Untagged enums pour choix structuraux** (EntityPointer, DateDef): Bon pattern quand justifié

✅ **Ordre préservé dans arrays**: Garantie naturelle pour CommonsDef/ColorsDef

## 5. Patterns à modifier

🔧 **SingleOrMultiple → Normaliser en Multiple**: Simplifie schéma et élimine fixes

🔧 **SaintCount → Ajouter schema_with**: Élimine fix manuel, meilleure maintenabilité
