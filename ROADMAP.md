# 🗺️ RemyLang - Roadmap de Développement

## Vue d'ensemble
Cette roadmap détaille les étapes de développement de RemyLang, de l'implémentation basique à un langage complet et fonctionnel.

---

## 📅 Phase 0 : Fondations (En cours)
**Objectif** : Mettre en place l'infrastructure de base

- [x] Créer le projet Rust
- [x] Définir la structure des dossiers
- [x] Configurer le main.rs basique
- [ ] Définir la syntaxe cible du langage
- [ ] Créer les structures de base (Token, AST)

**Livrable** : Architecture du projet claire et documentée

---

## 📅 Phase 1 : Lexer (Tokenisation)
**Objectif** : Convertir du texte brut en tokens

### 1.1 Tokens de base
- [ ] Créer `src/lexer/token.rs`
  - Token enum (Number, Identifier, Keywords, Operators)
  - Position tracking (ligne, colonne)
  - Tests unitaires pour Token

### 1.2 Lexer simple
- [ ] Créer `src/lexer/lexer.rs`
  - Struct Lexer avec itérateur
  - Reconnaissance des nombres (entiers)
  - Reconnaissance des opérateurs : `+`, `-`, `*`, `/`
  - Whitespace handling
  - Tests : "1 + 2" → [Number(1), Plus, Number(2), EOF]

### 1.3 Extension du Lexer
- [ ] Identifiants (variables)
- [ ] Keywords : `let`, `if`, `else`, `while`, `func`, `return`
- [ ] Opérateurs de comparaison : `==`, `!=`, `<`, `>`, `<=`, `>=`
- [ ] Ponctuation : `(`, `)`, `{`, `}`, `,`, `;`
- [ ] Commentaires (ligne et bloc)
- [ ] Strings literals
- [ ] Gestion d'erreurs avec position

**Livrable** : Lexer complet qui tokenise du code RemyLang

---

## 📅 Phase 2 : AST (Abstract Syntax Tree)
**Objectif** : Définir la représentation interne du programme

### 2.1 Expressions
- [ ] Créer `src/ast/expr.rs`
  - Literal (Number, String, Bool)
  - Binary (left, op, right)
  - Unary (op, right)
  - Variable (name)
  - Grouping (parenthèses)

### 2.2 Statements
- [ ] Créer `src/ast/stmt.rs`
  - ExpressionStmt
  - variable declaration
  - Block ({ ... })
  - If/Else
  - While
  - Return

### 2.3 Types (si langage typé)
- [ ] Créer `src/ast/types.rs`
  - Type enum (Int, Float, String, Bool, Function, etc.)
  - Type annotations

**Livrable** : AST complet et bien structuré

---

## 📅 Phase 3 : Parser (Analyse Syntaxique)
**Objectif** : Transformer tokens en AST

### 3.1 Parser basique
- [ ] Créer `src/parser/parser.rs`
  - Struct Parser
  - Méthodes de base : peek(), advance(), expect()
  - Gestion d'erreurs

### 3.2 Parsing des expressions
- [ ] Créer `src/parser/expr.rs`
  - Literals
  - Primary expressions
  - Pratt parser pour précédence des opérateurs
  - Parenthèses
  - Tests : "1 + 2 * 3" → AST correct

### 3.3 Parsing des statements
- [ ] Créer `src/parser/stmt.rs`
  - Let statements
  - If/Else statements
  - While loops
  - Blocks
  - Return statements

### 3.4 Parsing avancé
- [ ] Function declarations
- [ ] Function calls
- [ ] Arrays/Lists (optionnel)
- [ ] Error recovery (continuer après une erreur)

**Livrable** : Parser complet qui produit un AST valide

---

## 📅 Phase 4 : VM/Interpréteur (Exécution)
**Objectif** : Exécuter le code RemyLang

### 4.1 Interpreter basique (Tree-walking)
- [ ] Créer `src/vm/interpreter.rs`
  - Visitor pattern pour parcourir l'AST
  - Évaluation des expressions arithmétiques
  - Tests : "1 + 2" → 3

### 4.2 Variables et environnement
- [ ] Environment/Scope management
  - Stack de scopes
  - Variable storage
  - Variable lookup
- [ ] Let statements
- [ ] Variable assignment

### 4.3 Control flow
- [ ] If/Else execution
- [ ] While loops
- [ ] Break/Continue (optionnel)

### 4.4 Fonctions
- [ ] Function definitions
- [ ] Function calls
- [ ] Return values
- [ ] Closures (avancé)

**Livrable** : Interpréteur fonctionnel capable d'exécuter des programmes RemyLang

---

## 📅 Phase 5 : Standard Library
**Objectif** : Fournir des fonctions utilitaires de base

- [ ] Créer `src/stdlib/mod.rs`
- [ ] I/O : `print()`, `println()`, `input()`
- [ ] Math : `abs()`, `max()`, `min()`, `sqrt()` (optionnel)
- [ ] String : `len()`, `concat()`, `substring()` (optionnel)
- [ ] Type conversion : `to_string()`, `to_int()`, etc.

**Livrable** : Bibliothèque standard minimale

---

## 📅 Phase 6 : REPL (Read-Eval-Print Loop)
**Objectif** : Mode interactif pour tester le langage

- [ ] Créer `src/repl/mod.rs`
- [ ] Boucle de lecture
- [ ] Évaluation ligne par ligne
- [ ] Historique des commandes
- [ ] Pretty printing des résultats
- [ ] Help et commandes spéciales (`:quit`, `:help`, etc.)

**Livrable** : REPL fonctionnel comme Python ou Ruby

---

## 📅 Phase 7 : Gestion des Erreurs
**Objectif** : Messages d'erreur clairs et utiles

- [ ] Error types structurés
  - LexerError
  - ParseError
  - RuntimeError
- [ ] Messages d'erreur détaillés avec position
- [ ] Stack traces
- [ ] Suggestions de correction (did you mean?)
- [ ] Error recovery

**Livrable** : Système d'erreurs robuste et user-friendly

---

## 📅 Phase 8 : Tests et Exemples
**Objectif** : Validation et documentation par l'exemple

### 8.1 Tests unitaires
- [ ] Tests du Lexer (100+ tests)
- [ ] Tests du Parser (100+ tests)
- [ ] Tests de la VM (100+ tests)
- [ ] Tests d'intégration

### 8.2 Exemples
- [ ] `examples/hello_world.remy`
- [ ] `examples/fibonacci.remy`
- [ ] `examples/factorial.remy`
- [ ] `examples/functions.remy`
- [ ] `examples/control_flow.remy`
- [ ] `examples/variables.remy`

**Livrable** : Suite de tests complète + exemples variés

---

## 📅 Phase 9 : Optimisations (Optionnel)
**Objectif** : Améliorer les performances

- [ ] Bytecode compiler (au lieu de tree-walking)
- [ ] Stack-based VM
- [ ] Constant folding
- [ ] Dead code elimination
- [ ] Tail call optimization

**Livrable** : Performance améliorée

---

## 📅 Phase 10 : Fonctionnalités Avancées (Optionnel)
**Objectif** : Features de langage moderne

- [ ] System de modules/imports
- [ ] Structures/Objects
- [ ] Pattern matching
- [ ] Génériques
- [ ] Async/await
- [ ] Memory management avancé
- [ ] FFI (Foreign Function Interface) vers Rust

**Livrable** : Langage riche en fonctionnalités

---

## 📅 Phase 11 : Tooling
**Objectif** : Écosystème de développement

- [ ] CLI complet
  - `remylang run file.remy`
  - `remylang repl`
  - `remylang check file.remy` (syntax check)
  - `remylang fmt file.remy` (formatter)
- [ ] LSP (Language Server Protocol) pour VS Code
- [ ] Syntax highlighting
- [ ] Debugger
- [ ] Package manager (optionnel)

**Livrable** : Expérience développeur complète

---

## 📅 Phase 12 : Documentation
**Objectif** : Documentation complète du langage

- [ ] README.md complet
- [ ] Language specification
- [ ] Tutorial pour débutants
- [ ] API reference
- [ ] Architecture documentation
- [ ] Contributing guide
- [ ] Changelog

**Livrable** : Documentation professionnelle

---

## 🎯 Milestones Principaux

### Milestone 1 : "Hello World" (Phases 1-4)
**Date cible** : À définir  
**Critère** : Peut exécuter `print("Hello, World!")`

### Milestone 2 : "Calculator" (Phases 1-4)
**Date cible** : À définir  
**Critère** : Peut faire des calculs avec variables

### Milestone 3 : "Turing Complete" (Phases 1-5)
**Date cible** : À définir  
**Critère** : Fonctions + loops + conditions = complet

### Milestone 4 : "Production Ready" (Phases 1-8)
**Date cible** : À définir  
**Critère** : Testé, documenté, stable

### Milestone 5 : "Modern Language" (Toutes phases)
**Date cible** : À définir  
**Critère** : Features avancées + tooling complet

---

## 📊 Priorisation

### Must Have (P0)
- Lexer complet
- Parser fonctionnel
- Interpréteur basique
- Variables et fonctions
- Control flow

### Should Have (P1)
- REPL
- Gestion d'erreurs robuste
- Standard library basique
- Tests complets

### Nice to Have (P2)
- Optimisations
- Features avancées
- LSP / Tooling
- Bytecode compiler

### Future (P3)
- Package manager
- Advanced type system
- Async/await

---

## 🔄 Méthodologie

**Approche itérative** :
1. Implémenter une feature minimale
2. Tester
3. Valider
4. Étendre
5. Recommencer

**Principe** : Toujours avoir une version fonctionnelle, même minimale.

---

## 📝 Notes

- Cette roadmap est évolutive et sera ajustée selon les besoins
- Les phases peuvent se chevaucher
- Prioriser la simplicité et la robustesse sur les features
- Documenter au fur et à mesure

---

**Version** : 1.0  
**Dernière mise à jour** : 3 Novembre 2025  
**Status** : Phase 0 en cours
