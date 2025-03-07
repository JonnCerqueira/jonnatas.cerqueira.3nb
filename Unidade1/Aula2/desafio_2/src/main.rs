

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
/// Usamos *mut Node<T> para indicar que é um ponteiro mutável.
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<*mut Node<T>>,
    length: usize,
}

impl<T> Queue<T> {
    /// Cria uma nova fila vazia.
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Adiciona um novo elemento no final da fila.
    pub fn enqueue(&mut self, elem: T) {
        // Cria um novo nó na heap com o elemento
        let mut new_node = Box::new(Node {
            elem,
            next: None,
        });

        // Obtemos um ponteiro cru para o novo nó.
        // O operador "as *mut Node<T>" converte a Box em um ponteiro mutável.
        let new_node_ptr: *mut Node<T> = &mut *new_node;

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
        self.length += 1;
    }

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
            
            self.length -= 1;
            // Retorna o elemento armazenado no nó removido.

            boxed_node.elem
        })
    }

    /// Retorna uma referência ao elemento da frente da fila sem removê-lo.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.elem)
    }

    /// Retorna o número atual de elementos na fila.
    pub fn len(&self) -> usize {
        self.length
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        // Enquanto houver um elemento na cabeça, removemos (dequeue) cada elemento.
        while let Some(_) = self.dequeue() {}
    }
}

// Módulo de testes: esse código será compilado e executado apenas quando rodarmos os testes.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();
        // "let mut queue = Queue::new();"
        //
        // - "let" declara uma nova variável.
        // - "mut" indica que essa variável é mutável, ou seja, pode ter seu valor alterado.
        // - "Queue::new()" chama o método associado "new" para criar uma nova instância da nossa fila.
        
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

    #[test]
    fn test_peek() {
        // Cria uma nova fila vazia.
        let mut queue = Queue::new();

        // Adiciona o elemento 1 na fila.
        queue.enqueue(1);
        // Adiciona o elemento 2 na fila.
        queue.enqueue(2);

        // Verifica se o elemento da frente da fila (sem removê-lo) é 1.
        // `peek` deve retornar uma referência ao primeiro elemento da fila.
        assert_eq!(queue.peek(), Some(&1));

        // Remove o elemento da frente da fila, que deve ser 1.
        // `dequeue` deve retornar o elemento removido.
        assert_eq!(queue.dequeue(), Some(1));

        // Verifica novamente o elemento da frente da fila, que agora deve ser 2.
        // `peek` deve retornar uma referência ao novo primeiro elemento da fila.
        assert_eq!(queue.peek(), Some(&2));
    }


    #[test]
    fn test_len() {
        // Cria uma nova fila vazia.
        let mut queue = Queue::new();

        // Verifica se o comprimento inicial da fila é 0, já que ela está vazia.
        assert_eq!(queue.len(), 0);

        // Adiciona o elemento 1 na fila.
        queue.enqueue(1);
        // Verifica se o comprimento da fila é 1, pois um elemento foi adicionado.
        assert_eq!(queue.len(), 1);

        // Adiciona o elemento 2 na fila.
        queue.enqueue(2);
        // Verifica se o comprimento da fila é 2, pois dois elementos foram adicionados.
        assert_eq!(queue.len(), 2);

        // Remove o elemento da frente da fila.
        queue.dequeue();
        // Verifica se o comprimento da fila é 1, pois um elemento foi removido.
        assert_eq!(queue.len(), 1);

        // Remove o último elemento da fila.
        queue.dequeue();
        // Verifica se o comprimento da fila é 0, pois todos os elementos foram removidos.
        assert_eq!(queue.len(), 0);
    }
}