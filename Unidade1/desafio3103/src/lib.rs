// Implementação SUPER SIMPLIFICADA de Árvores Binárias em Rust
// Versão corrigida que compila sem erros

// =========================================
// ÁRVORE BINÁRIA DE BUSCA (BST) - BÁSICA
// =========================================

// Estrutura básica de um nó
#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,               // Valor do nó
    left: Option<Box<Node>>,  // Filho esquerdo
    right: Option<Box<Node>>, // Filho direito
}

// Árvore BST
#[allow(dead_code)]
#[derive(Debug)]
struct BST {
    root: Option<Box<Node>>, // Raiz da árvore
}

impl BST {
    // Criar árvore vazia
    #[allow(dead_code)]
    fn new() -> Self {
        BST { root: None }
    }

    // Inserir valor
    #[allow(dead_code)]
    fn insert(&mut self, value: i32) {
        // Usando uma abordagem recursiva sem múltiplos empréstimos mutáveis
        let mut current = &mut self.root;

        // Loop para encontrar o lugar certo para inserir
        loop {
            match current {
                // Lugar vazio encontrado, inserimos aqui
                None => {
                    *current = Some(Box::new(Node {
                        value,
                        left: None,
                        right: None,
                    }));
                    return;
                }
                // Navegamos para a posição correta
                Some(node) => {
                    if value < node.value {
                        // Ir para esquerda
                        current = &mut node.left;
                    } else if value > node.value {
                        // Ir para direita
                        current = &mut node.right;
                    } else {
                        // Valor duplicado, nada a fazer
                        return;
                    }
                }
            }
        }
    }

    // Verifica se está vazia
    #[allow(dead_code)]
    fn is_empty(&self) -> bool {
        self.root.is_none()
    } 

    // Buscar valor
    #[allow(dead_code)]
    fn search(&self, value: i32) -> bool {
        let mut current = &self.root;

        // Loop para navegar até encontrar ou não o valor
        while let Some(node) = current {
            if value == node.value {
                return true; // Encontrou
            } else if value < node.value {
                current = &node.left; // Vai para esquerda
            } else {
                current = &node.right; // Vai para direita
            }
        }

        false // Não encontrou
    }
}

#[cfg(test)]
mod bst_tests {
    use super::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        let mut bst = BST::new();
        
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        assert!(!bst.search(20));
        assert!(!bst.is_empty());
    }
}

