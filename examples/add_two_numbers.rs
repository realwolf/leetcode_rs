#[derive(Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

fn main()
{
    let mut l1: ListNode = ListNode { val: 2, next: 
        Some(Box::new(ListNode{ val: 4, next: 
        Some(Box::new(ListNode{ val: 3, next: None}))}))};

    let mut l2: ListNode = ListNode { val: 5, next: 
        Some(Box::new(ListNode{ val: 6, next: 
        Some(Box::new(ListNode{ val: 4, next: None}))}))};

    println!("{:?}\r\n", add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2))));
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut result = Some(Box::new(ListNode { val: 0, next: None }));
    
    let mut p1: Option<Box<ListNode>> = l1.clone();
    let mut p2: Option<Box<ListNode>> = l2.clone();

    let mut vec_result: Vec<i32> = Vec::new();
    let mut carry = 0;
    let p = &result;

    loop 
    {
        let mut v1 = 0;
        let mut v2 = 0;
        let mut flag_readed = false;

        if let Some(node) = p1
        {
            v1 = node.val;
            p1 = node.next;
            flag_readed = true;
        }

        if let Some(node) = p2
        {
            v2 = node.val;
            p2 = node.next;
            flag_readed = true;
        }

        if flag_readed == false
        {
            if carry > 0
            {
                vec_result.push(carry);
            }
            break;
        }

        let mut sum = v1 + v2 + carry;

        if sum >= 10 
        {
            carry = 1;
            sum = sum - 10;
        }

        vec_result.push(sum);
        if let Some(v) = p{
            v.val = sum;
            p = Some(v.next);
        }
    }

    return Some(Box::new(generate_list_node(&mut vec_result, vec_result)));
}


pub fn generate_list_node(input: Option<Box<ListNode>>, vec_val: &Vec<i32>) -> Option<Box<ListNode>>
{
    
}
    