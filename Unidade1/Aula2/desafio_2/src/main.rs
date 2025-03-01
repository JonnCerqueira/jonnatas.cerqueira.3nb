

/// Um nó da fila.
/// Cada nó contém um elemento e uma referência para o próximo nó.
pub struct Node<T> {
    // O elemento armazenado no nó.
    elem: T,
    // Ponteiro para o próximo nó.
    // Utilizamos Option<Box<Node<T>>> para indicar que pode haver (Some) ou não (None) um próximo nó.
    next: Option<Box<Node<T>>>,
}

/// A fila (queue) propriamente dita.
/// Mantém um ponteiro para o primeiro nó (head) e para o último nó (tail).
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    // tail será um ponteiro cru (raw pointer) para o último nó da fila.
    // Usamos *mut Node<T> para indicar que é um ponteiro mutável.
    tail: Option<*mut Node<T>>,
}

impl<T> Queue<T> {
    /// Cria uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
        }
    }
}    
    impl<T> Queue<T> {
        /// Adiciona um novo elemento no final da fila.
        pub fn enqueue(&mut self, elem: T) {
            // Cria um novo nó na heap com o elemento.
            let mut new_node = Box::new(Node {
                elem,
                next: None,
            });
    
            // Obtemos um ponteiro cru para o novo nó.
            // O operador "as *mut Node<T>" converte a Box em um ponteiro mutável.
            let new_node_ptr: *mut Node<T> = &mut *new_node;
    
            // Se a cauda existe, significa que a fila não está vazia.
            if let Some(tail_ptr) = self.tail {
                unsafe {
                    // Acessa o nó da cauda e atualiza seu campo next para apontar para o novo nó.
                    (*tail_ptr).next = Some(new_node);
                }
            } else {
                // Se a fila está vazia, a nova node será também a cabeça.
                self.head = Some(new_node);
            }
            // Atualiza a cauda da fila para ser o novo nó.
            self.tail = Some(new_node_ptr);
    
            // Se a fila estava vazia, precisamos também definir a cabeça.
            if self.head.is_none() {
                self.head = Some(unsafe { Box::from_raw(new_node_ptr) });
            }
        }
    }

    impl<T> Queue<T> {
        /// Remove e retorna o elemento da cabeça da fila.
        pub fn dequeue(&mut self) -> Option<T> {
            // Tira (take) a cabeça da fila, substituindo-a por None.
            self.head.take().map(|boxed_node| {
                // Se existir um próximo nó, atualiza a cabeça para ele.
                self.head = boxed_node.next;
    
                // Se a nova cabeça for None, a fila ficou vazia e a cauda também deve ser None.
                if self.head.is_none() {
                    self.tail = None;
                }
                // Retorna o elemento armazenado no nó removido.
                boxed_node.elem
            })
        }
    }

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // Enquanto houver um elemento na cabeça, removemos (dequeue) cada elemento.
        while let Some(_) = self.dequeue() {}
    }
}

// src/lib.rs

// Módulo de testes: esse código será compilado e executado apenas quando rodarmos os testes.
#[cfg(test)]
mod tests {
    // Traz para o módulo as definições do escopo superior (como a estrutura Queue) para os testes.
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        // "let mut queue = Queue::new();"
        //
        // - "let" declara uma nova variável.
        // - "mut" indica que essa variável é mutável, ou seja, pode ter seu valor alterado.
        // - "Queue::new()" chama o método associado "new" para criar uma nova instância da nossa fila.
        let mut queue = Queue::new();

        // Adiciona (enqueue) elementos na fila.
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        // Como uma fila funciona em modo FIFO (First In, First Out),
        // o primeiro elemento inserido deve ser o primeiro a sair.
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        // Quando a fila estiver vazia, dequeue deve retornar None.
        assert_eq!(queue.dequeue(), None);
    }
}
