use std::{rc::Rc, collections::{HashMap, LinkedList}, cell::RefCell};
type FibHeapNodeType = Rc<RefCell<FibHeapNode>>;

struct FibHeapNode
{
    value: u8,
    marked: bool,
    key: (usize, usize, usize),
    parent: Option<FibHeapNodeType>,
    children: LinkedList<FibHeapNodeType>
}

impl FibHeapNode
{
    pub fn new(priority: u8, position: (usize, usize, usize), parent: Option<FibHeapNodeType>) -> FibHeapNodeType
    {
        Rc::new(RefCell::new(FibHeapNode {
            value: priority,
            marked: false,
            key: position,
            parent: parent,
            children: LinkedList::<FibHeapNodeType>::new()
        }))
    }

    pub fn rank(&self) -> usize
    {
        self.children.len()
    }

    pub fn set_parent(& mut self, parent: Option<FibHeapNodeType>) -> ()
    {
        self.parent = parent
    }


}

impl PartialEq for FibHeapNode
{
    fn eq(&self, other: &FibHeapNode) -> bool {
        self.key == other.key
    }
}

impl Eq for FibHeapNode {}


pub struct FibHeap
{
    map: HashMap<(usize,usize,usize), FibHeapNodeType>,
    roots: LinkedList<FibHeapNodeType>,
    min: Option<FibHeapNodeType>
}

impl FibHeap
{
    pub fn new() -> FibHeap
    {
        FibHeap{
            map: HashMap::new(),
            roots: LinkedList::new(),
            min: None
        }
    }

    pub fn insert(& mut self, position: (usize, usize, usize), value: u8)
    {
        let node = FibHeapNode::new(value, position, None);
        self.map.insert(position, node.clone());
        match self.min{
            Some(ref m) => {
                self.roots.push_front(node.clone());
                if m.borrow().value > node.borrow().value
                {
                    self.min = Some(node.clone());
                }
            },
            None => {
                self.roots.push_front(node.clone());
                self.min = Some(node.clone());
            }
        }
    }

    pub fn pop_min(& mut self) -> Option<(usize, usize, usize)>
    {
        match self.min{
            Some(ref m) => {
                let result = Some(m.borrow().key);
                {
                    for child in &m.borrow().children
                    {
                        child.clone().borrow_mut().set_parent(None);
                    }
                }
                {
                    self.roots.append(&mut m.borrow_mut().children);
                }
                let index = self.roots.iter().position(|a| a.borrow().key == m.borrow().key).unwrap();
                self.roots.remove(index);
                self.map.remove(&m.borrow().key);

                if self.roots.is_empty()
                {
                    self.min = None;
                }
                else
                {
                    let mut new_min = self.roots.front().unwrap().clone();
                    for node in self.roots.iter().skip(1)
                    {
                        if new_min.borrow().value > node.borrow().value
                        {
                            new_min = node.clone();
                        }
                    }
                    self.min = Some(new_min.clone());
                }

                self.consolidate();

                result
            },
            None => {
                None
            }
        }
    }

    fn insert_or_merge(ranked: & mut HashMap<usize, FibHeapNodeType>, node: FibHeapNodeType) -> ()
    {
        if ranked.contains_key(&node.borrow().rank())
        {
            let other_node = ranked.remove(&node.borrow().rank()).unwrap();
            if other_node.borrow().value > node.borrow().value
            {
                {
                    let mut m = node.borrow_mut();
                    m.children.push_front(other_node.clone());
                    m.marked = false;
                }
                other_node.borrow_mut().set_parent(Some(node.clone()));
                Self::insert_or_merge(ranked, node.clone());
            }
            else
            {
                {
                    let mut m = other_node.borrow_mut();
                    m.children.push_front(node.clone());
                    m.marked = false;
                }
                node.borrow_mut().set_parent(Some(other_node.clone()));
                Self::insert_or_merge(ranked, other_node.clone());
            }

        }
        else {
            ranked.insert(node.borrow().rank(), node.clone());
        }
    }

    fn consolidate(& mut self) -> ()
    {
        let mut ranked = HashMap::<usize, FibHeapNodeType>::new();
        for node in &self.roots
        {
            if node.borrow().key != self.min.clone().unwrap().borrow().key
            {
                Self::insert_or_merge(&mut ranked, node.clone());
            }
        }
        self.roots = ranked.values().map(|x| x.clone())
                           .collect();
        if let Some(m) = self.min.clone(){
            self.roots.push_front(m.clone());
        }
    }

    pub fn decrease_key(& mut self, key: (usize, usize, usize), value: u8) -> ()
    {
        let node = self.map.get(&key).unwrap().clone();
        {
            node.borrow_mut().value = value;
        }
        let parent = node.borrow().parent.clone();
        if let Some(parent) = parent {
            if parent.borrow().value > node.borrow().value
            {
                //need to get order right
                self.cut(node.clone(), parent.clone());
                self.cascade_cut(parent)
            }
            else {
                //order is preserved
            }
        }
        if self.min.clone().unwrap().borrow().value > value
        {
            //update min
            self.min = Some(node);
        }
    }

    fn cut(&mut self, child: FibHeapNodeType, parent: FibHeapNodeType) -> () {
        {
            let index = parent.borrow().children.iter().position(|n| n.borrow().key == child.borrow().key).unwrap();
            let p_children = &mut parent.borrow_mut().children;
            p_children.remove(index);
        }
        {
            let mut c = child.borrow_mut();
            c.set_parent(None);
            c.marked = false;
        }
        self.roots.push_front(child);
    }

    fn cascade_cut(&mut self, node: FibHeapNodeType) -> ()
    {
        let parent = node.borrow().parent.clone();
        if let Some(p) = parent {
            if p.borrow().marked
            {
                self.cut(node, p.clone());
                self.cascade_cut(p);
            }
            else
            {
                node.borrow_mut().marked = true;
            }
        }
    }
}