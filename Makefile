# Variables
CARGO = cargo
TEST_FLAGS = --color=always
NOCAPTURE_FLAGS = -- --nocapture
BENCH_FLAGS = -- --bench
PROJECT_NAME = $(shell grep '^name' Cargo.toml | head -1 | cut -d '"' -f 2)

# Couleurs pour l'affichage
GREEN = \033[0;32m
RED = \033[0;31m
YELLOW = \033[0;33m
BLUE = \033[0;34m
NC = \033[0m # No Color

.PHONY: all test unit integration detailed clean help bench doc tarpaulin docs docs-build docs-serve docs-deploy

# Cible par défaut
all: test

# Tous les tests
test: unit integration
	@echo "$(GREEN)✓ Tous les tests sont passés !$(NC)"

# Tests unitaires seulement
unit:
	@echo "$(BLUE)Exécution des tests unitaires...$(NC)"
	@$(CARGO) test $(TEST_FLAGS)

# Tests d'intégration seulement
integration:
	@echo "$(BLUE)Exécution des tests d'intégration...$(NC)"
	@$(CARGO) test --test integration_tests $(TEST_FLAGS)
	@$(CARGO) test --test matrix_operations $(TEST_FLAGS)

# Tests avec output détaillé
detailed:
	@echo "$(BLUE)Exécution des tests avec output détaillé...$(NC)"
	@$(CARGO) test $(NOCAPTURE_FLAGS)

# Nettoyage
clean:
	@echo "$(YELLOW)Nettoyage du projet...$(NC)"
	@$(CARGO) clean

# Aide
help:
	@echo "$(GREEN)Makefile pour le projet $(PROJECT_NAME)$(NC)"
	@echo ""
	@echo "Cibles disponibles:"
	@echo "  $(GREEN)all$(NC)        - Exécute tous les tests (cible par défaut)"
	@echo "  $(GREEN)test$(NC)       - Exécute les tests unitaires et d'intégration"
	@echo "  $(GREEN)unit$(NC)       - Exécute seulement les tests unitaires"
	@echo "  $(GREEN)integration$(NC)- Exécute seulement les tests d'intégration"
	@echo "  $(GREEN)detailed$(NC)   - Exécute les tests avec output détaillé"
	@echo "  $(GREEN)bench$(NC)      - Exécute les benchmarks"
	@echo "  $(GREEN)doc$(NC)        - Génère la documentation"
	@echo "  $(GREEN)tarpaulin$(NC)  - Rapport de couverture de code (si installé)"
	@echo "  $(GREEN)clean$(NC)      - Nettoie le projet"
	@echo "  $(GREEN)help$(NC)       - Affiche cette aide"
	@echo ""
	@echo "Exemples:"
	@echo "  make              # Exécute tous les tests"
	@echo "  make unit         # Tests unitaires seulement"
	@echo "  make detailed     # Tests avec output complet"

# Benchmarks (si vous en avez)
bench:
	@echo "$(BLUE)Exécution des benchmarks...$(NC)"
	@$(CARGO) bench $(BENCH_FLAGS)

# Documentation
doc:
	@echo "$(BLUE)Génération de la documentation...$(NC)"
	@$(CARGO) doc --open

# Couverture de code avec tarpaulin
tarpaulin:
	@echo "$(BLUE)Vérification de l'installation de tarpaulin...$(NC)"
	@which cargo-tarpaulin > /dev/null 2>&1 || (echo "$(RED)Tarpaulin n'est pas installé. Installation...$(NC)" && cargo install cargo-tarpaulin)
	@echo "$(BLUE)Génération du rapport de couverture...$(NC)"
	@cargo tarpaulin --ignore-tests --out Html

# Test avec filtrage (pour exécuter un test spécifique)
test-%:
	@echo "$(BLUE)Exécution des tests correspondant à '$*'...$(NC)"
	@$(CARGO) test --test $* $(TEST_FLAGS)

# Version verbose pour le débogage
verbose: TEST_FLAGS += --verbose
verbose: test

# Version release pour les tests de performance
release-test:
	@echo "$(BLUE)Exécution des tests en mode release...$(NC)"
	@$(CARGO) test --release $(TEST_FLAGS)

# Vérification du code sans exécution des tests
check:
	@echo "$(BLUE)Vérification du code...$(NC)"
	@$(CARGO) check
	@$(CARGO) clippy

# Installation des dépendances de développement
setup:
	@echo "$(BLUE)Installation des outils de développement...$(NC)"
	@rustup component add clippy
	@rustup component add rustfmt
	@cargo install cargo-tarpaulin 2>/dev/null || echo "$(YELLOW)tarpaulin déjà installé ou échec d'installation$(NC)"

# Build documentation
docs-build:
	@echo "$(BLUE)Building documentation...$(NC)"
	@./scripts/build_docs.sh

# Serve documentation locally
docs-serve:
	@echo "$(BLUE)Serving documentation locally...$(NC)"
	@cd docs && mkdocs serve

# Deploy documentation
docs-deploy:
	@echo "$(BLUE)Deploying documentation...$(NC)"
	@cd docs && mkdocs gh-deploy

# Open documentation
docs-open:
	@echo "$(BLUE)Opening documentation...$(NC)"
	@open docs/site/index.html