**Variables**

Définition:

<type> <name> = <value>

*Entier:*

```
Int nb = 42;
```

*Char:*

```
Char c = 'c';
```

*Bool:*

```
Bool isOk = True;
```

*Str:*

```
String str = "Hello World!";
```

*Array:*

```
Array<type> list = [];
Array<Int> intList = [1, 2, 3];
Array<String> strList = ["foo", "bar", "baz"];

// Accès aux éléments
Int first = intList[0];  // Premier élément
intList[1] = 42;         // Modification
```

*Tuple:*

```
// Pour l'instant : non implémenté
// Sera ajouté dans une version future
```

**Reassignment**

Pour modifier une variable existante, pas besoin de redéclarer le type :

```
Int nb = 42;
nb = 100;        // Réassignation simple

String str = "Hello";
str = "World";   // Nouvelle valeur
```

**Compound Assignment (optionnel pour v1)**

```
Int a = 10;
a += 5;   // équivaut à : a = a + 5;
a -= 2;   // équivaut à : a = a - 2;
a *= 3;   // équivaut à : a = a * 3;
a /= 2;   // équivaut à : a = a / 2;
a %= 3;   // équivaut à : a = a % 3;
```

**Functions**

Definition:

func <name>(<params(optionnals)>) → <ret_value>

No params. No return value func :

```
func HelloWorld() {
	print("Hello, World!");
}
```

One Param. No return value func :

```
func PrintStr(String str) {
	print(str);
}
```

Params. No return value func :

```
func PrintBoth(String str, Int nb) {
	print(str);
	print(nb);
}
```

Params. Return value func :

```
func Add(Int a, Int b) -> Int {
	Int c = a + b;
	return c;
}
```

**Arithmetic Operations**

add : +

sub : -

mul : *

pow : **

div : /

mod : %

**Logical Operations**

and : &&
or : ||
not : !

**Comparisons :**

equal : ==

not equal : !=

less than : <

more than : >

less or equal : <=
more or equal : >=

**Conditions :**

Structure :

```
if (condition) {
    // code
}

if (condition) {
    // code
} else {
    // code
}

if (condition1) {
    // code
} else if (condition2) {
    // code
} else {
    // code
}
```

Exemple avec opérateurs logiques :

```
Int age = 25;
Bool isStudent = True;

if (age >= 18 && isStudent) {
    print("Adulte étudiant");
} else if (age >= 18) {
    print("Adulte");
} else {
    print("Mineur");
}
```

**Loops :**

Non implémentés pour l'instant (Phase 2 du projet).

Planifié :
- `while (condition) { ... }`
- `for (init; condition; increment) { ... }` (peut-être)
- `for item in array { ... }` (peut-être)

**Comments**

Commentaire sur une ligne :
```
// Ceci est un commentaire
Int nb = 42;  // Commentaire de fin de ligne
```

Commentaire multiligne :
```
/*
 * Ceci est un commentaire
 * sur plusieurs lignes
 */
```

**Statements & Semicolons**

Règles :
- Les statements (déclarations, assignations, appels de fonction, return) se terminent par `;`
- Les blocs de code `{ }` n'ont PAS de `;` après
- Les définitions de fonctions n'ont PAS de `;` après

```
Int a = 42;           // ✓ avec ;
print(a);             // ✓ avec ;
return 0;             // ✓ avec ;

if (a > 10) {         // ✗ pas de ; après {
    print("Grand");   // ✓ avec ;
}                     // ✗ pas de ; après }

func Test() {         // ✗ pas de ; après {
    // code
}                     // ✗ pas de ; après }
```

**Simple Code Example :**

```
func Add(Int a, Int b) -> Int {
	return a + b;
}

func Main() -> Int {
	Int a = 12;
	a = Add(a, 12);      // Notez la majuscule à Add (nom de fonction)
	print(a);            // Affiche : 24
	
	// Exemple avec condition
	if (a > 20) {
		print("a est grand");
	} else {
		print("a est petit");
	}
	
	return 0;
}
```

**Built-in Functions (stdlib)**

Fonctions de base disponibles :

```
print(value)        // Affiche une valeur avec retour à la ligne
input()            // Lit une entrée utilisateur (retourne String)
len(array)         // Retourne la longueur d'un Array
```

Exemples :
```
print("Hello");
String name = input();
Array<Int> list = [1, 2, 3];
Int size = len(list);  // 3
```

**Advanced Features (Phase 2+)**

Fonctionnalités prévues pour plus tard :
- Treat String as arrays of char (index logic: `str[0]`)
- Memory allocation and handling
- Modules/imports
- Structures/Objects
- Pattern matching
- Error handling (try/catch)

**Design Decisions**

Choix importants :
- **Types explicites** : Toujours déclarer le type (`Int a = 42`)
- **Case-sensitive** : `Add` ≠ `add`, `True` ≠ `true`
- **Types en majuscule** : `Int`, `String`, `Bool`, `Char`, `Array`
- **Booleans en majuscule** : `True`, `False`
- **Semicolons obligatoires** : Après chaque statement
- **Parenthèses obligatoires** : Dans les conditions `if ()`
- **Strongly typed** : Pas de conversion implicite entre types