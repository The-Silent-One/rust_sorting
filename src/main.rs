use std::time::Instant;

fn random_vector(size: usize) -> Vec<i16>{
    let mut arr: Vec<i16> = Vec::new();
    for _ in 0..size{
        arr.push(rand::random::<i16>())
    }
    return arr;
}

fn run_benchmark(batch: usize,size: usize, sort: fn(Vec<i16>) -> Vec<i16>){
    for _ in 0..batch{
        let mut vec: Vec<i16> = random_vector(size);
        //println!("{:?}",vec);
        vec = sort(vec);
        //println!("{:?}",vec);
        assert_sorted(&vec);
    }
}

pub fn assert_sorted(arr: &[i16]){
    for i in 1..arr.len(){
        assert!(arr[i-1] <= arr[i]);
    }
}

pub fn heap_sort(arr: Vec<i16>) -> Vec<i16>{

    fn i_left_child(i: usize) -> usize{
        return core::cmp::min(usize::MAX,2*i+1);
    }

    let mut result = arr.to_vec();
    let count : usize = arr.len();
    let mut start: usize = count/2;
    let mut end : usize = count;
    let mut root: usize;
    let mut child: usize;
    
    while end > 1 {
        if start > 0 {
            start -=1;
        } else {
            end -=1;
            result.swap(end, 0);
        }
        root = start;

        while i_left_child(root) < end {
            child = i_left_child(root);
            if child+1 < end && result[child] < result[child+1] {
                child +=1;
            }

            if result[root] < result[child] {
                result.swap(root, child);
                root = child;
            } else {
                break;
            }
        }

    }

    return result;

}

pub fn quick_sort(arr: Vec<i16>) -> Vec<i16>{
    
    if arr.len() < 2 {
        return arr;
    }
    let pivot: i16 = arr[0];
    let (left, right) :(Vec<i16>,Vec<i16>) = arr[1..].iter().partition(|x| *x < &pivot);
    return [quick_sort(left),vec![pivot],quick_sort(right)].concat();
}

pub fn merge_sort(arr: Vec<i16> ) -> Vec<i16>{
    
    fn merge(mut arr1: Vec<i16>, mut arr2: Vec<i16>) -> Vec<i16>  {
        let mut result : Vec<i16> = Vec::new();
        while !arr1.is_empty() && !arr2.is_empty() {
            if arr1.first() <= arr2.first() {
                result.push(arr1.remove(0));
                
            } else {
                result.push(arr2.remove(0));
            }
        }
        return [result,arr1,arr2].concat();
    }

    if arr.len() <= 1 {
        return arr.to_vec();
    }
    let split_index : usize = arr.len() / 2; 
    let res: (&[i16], &[i16]) = arr.split_at(split_index);
    let ( mut left,  mut right ) : (Vec<i16>,Vec<i16>) = (res.0.to_vec(), res.1.to_vec());
    left = merge_sort(left); 
    right = merge_sort(right);
    return merge(left.to_vec(), right.to_vec());
}

pub fn insertion_sort(arr: Vec<i16>) -> Vec<i16>{
    let mut res: Vec<i16> = arr.to_vec();
    let mut i : usize = 0;
    let mut j : usize;
    while i < res.len() {
        j = i;
        while j > 0 && res[j-1] > res[j] {
            res.swap(j, j-1);
            j -=1;
        }
        i +=1;
    }
    return res;
}

pub fn intro_sort(arr: Vec<i16>) -> Vec<i16>{

    fn sort(arr: Vec<i16>,max_depth: u32) -> Vec<i16> {
        if arr.len() < 16 {
            return insertion_sort(arr);
        } 
        else if max_depth == 0 {
            return heap_sort(arr);
        }
        else {
            let pivot: i16 = arr[0];
            let (left, right) :(Vec<i16>,Vec<i16>) = arr[1..].iter().partition(|x| *x < &pivot);
            return [sort(left,max_depth-1),sort(right,max_depth-1)].concat()
        }
    }

    let max_depth: u32 = arr.len().ilog10() * 2 ;
    return sort(arr,max_depth);
}

pub fn selection_sort(arr : Vec<i16>) -> Vec<i16> {
    let mut res: Vec<i16> = arr.to_vec();
    let mut jmin: usize;

    for i in 0..res.len() {
        jmin = i;
        for j in i+1..res.len() {
            if res[j] < res[jmin] {
                jmin = j;
            }
        }
        if jmin !=i {
            res.swap(i, jmin);
        }
    }
    return res;
}

pub fn patience_sort(arr: Vec<i16>) -> Vec<i16> {
    
    fn min_heap(piles : &mut Vec<Vec<i16>>) -> i16 {
        let mut min: i16 = *piles.last().unwrap().last().unwrap();
        for i in 0..piles.len() {
            if !(piles[i].is_empty()) && min >= *piles[i].last().unwrap() {
                min = piles[i].pop().unwrap();
            }
        }
        piles.retain(|x| !x.is_empty());
        return min;
    }
    
    let mut piles: Vec< Vec<i16>> = Vec::new();
    let mut res: Vec<i16> = Vec::new();
    let mut j: usize;

    piles.push(vec![arr[0]]);
    for i in 1..arr.len() {
        j = 0;
        while j < piles.len() {
            if arr[i] <= *piles[j].last().unwrap() {
                piles[j].push(arr[i]);
                break;
            }
            j+= 1;
        }
        if j == piles.len() {
            piles.push(vec![arr[i]]);
        }
    }
    loop {
        res.push(min_heap(&mut piles));
        if piles.is_empty() {
            break;
        }
    }

    return res;

}

pub fn bubble_sort(arr: Vec<i16>) -> Vec<i16> {
    let mut res: Vec<i16> = arr.to_vec();
    let mut swapped: bool = true;

    while swapped {
        swapped = false;
        for i in 1..res.len() {
            if res[i-1] > res[i] {
                res.swap(i-1, i);
                swapped = true;
            }
        }
    }

    return res;
}

pub fn optimized_bubble_sort(arr: Vec<i16>) -> Vec<i16> {
    let mut res: Vec<i16> = arr.to_vec();
    let mut newn: usize;
    let mut n: usize = res.len();

    while n >1 {
        newn = 0;
        for i in 1..n {
            if res[i-1] > res[i] {
                res.swap(i-1, i);
                newn = i;
            }
        }
        n = newn;
    }
    return res;
}

pub fn cocktail_shaker_sort(arr: Vec<i16>) -> Vec<i16> {
    let mut res: Vec<i16> = arr.to_vec();
    let mut begin_id : usize = 1;
    let mut end_id: usize = res.len();
    let mut new_begin_id: usize;
    let mut new_end_id: usize;
    while begin_id <= end_id {
        new_begin_id = begin_id;
        new_end_id = end_id;
        for i in begin_id..end_id {
            if res[i-1] > res[i] {
                res.swap(i, i-1);
                new_end_id = i;
            }
        }
        end_id = new_end_id;
        for i in (begin_id..end_id).rev() {
            if res[i-1] > res[i] {
                res.swap(i, i-1);
                new_begin_id = i;
            }
        }
        begin_id = new_begin_id+1;
    }
    
    return res;
} 

pub fn gnome_sort(arr: Vec<i16>) -> Vec<i16> {
    let mut pos:usize=0;
    let mut res = arr.to_vec();
    while pos < arr.len() {
        if pos == 0 || res[pos] >= res[pos-1] {
            pos +=1;
        } else {
            res.swap(pos, pos-1);
            pos -=1;
        }
    }
    return res;
}

pub fn odd_even_sort(arr: Vec<i16>) -> Vec<i16> {
    let mut res = arr.to_vec();
    let mut sorted: bool = false;
    while !sorted {
        sorted = true;
        for i in (1..res.len()-1).step_by(2) {
            if res[i] > res[i+1] {
                res.swap(i, i+1);
                sorted = false;
            }
        }
        for i in (0..res.len()-1).step_by(2) {
            if res[i] > res[i+1] {
                res.swap(i, i+1);
                sorted = false;
            }
        }
    }
    return res;
}

pub fn pancake_sort(arr: Vec<i16>) -> Vec<i16> {
    fn flip( arr: &mut Vec<i16>, mut k: usize) {
        let mut left: usize =0;
        while left < k {
            arr.swap(k,left);
            k -=1;
            left +=1;
        }
    }
    
    fn get_max_index(arr: &mut Vec<i16>, k: usize) -> usize{
        let mut index:usize =0;
        for i in 0..k {
            if arr[i] > arr[index] {
                index=i;
            }
        }
        return index;
    }
    
    let mut res = arr.to_vec();
    let mut n: usize = res.len();
    let mut max_index: usize;
    while n > 1 {
        max_index = get_max_index(&mut res, n);
        if max_index != n-1 {
            flip(&mut res, max_index);
            flip(&mut res, n-1);
        }
        n -=1;
    }
    return res;
}

pub fn strand_sort(arr: Vec<i16>) -> Vec<i16> {
    fn merge(arr1: &mut Vec<i16>, arr2: &mut Vec<i16>) -> Vec<i16>{
        let mut res: Vec<i16> = Vec::new();
        while !arr1.is_empty() && !arr2.is_empty() {
            if arr1.first() < arr2.first() {
                res.push(arr1.remove(0));
            } else {
                res.push(arr2.remove(0));
            }
        }
        res.append(arr1);
        res.append(arr2);
        return res;
    }

    if arr.is_empty() {
        return Vec::new();
    }

    let mut tmp_arr: Vec<i16> = arr.to_vec();
    let mut sublist:Vec<i16> = Vec::new();
    sublist.push(tmp_arr.remove(0));

    let mut i:usize = 0;

    while  i < tmp_arr.len() && !tmp_arr.is_empty() {
        if tmp_arr[i] > *sublist.last().unwrap() {
            sublist.push(tmp_arr.remove(i));
        } else {
            i +=1;
        }      
    }
    let mut remaining_arr: Vec<i16> = strand_sort(tmp_arr);
    return merge(&mut sublist, &mut remaining_arr);
        
}

fn main() {
    let sorts: [(fn(Vec<i16>) -> Vec<i16>, &str); 13] = [
        (heap_sort,"Heap Sort"),
        (quick_sort,"Quick Sort"),
        (merge_sort,"Merge Sort"),
        (insertion_sort,"Insertion Sort"),
        (intro_sort,"Intro Sort"),
        (patience_sort,"Patience Sort"),
        (bubble_sort,"Bubble Sort"),
        (optimized_bubble_sort,"Optimized Bubble Sort"),
        (cocktail_shaker_sort,"Cocktail Shaker Sort"),
        (gnome_sort,"Gnome Sort"),
        (odd_even_sort,"Odd Even Sort"),
        (pancake_sort,"Pancake Sort"),
        (strand_sort,"Strand Sort")
    ];
    
    for sort in sorts{
        let now: Instant = Instant::now();
        run_benchmark(1_000,1_000,sort.0);
        println!("{} Elapsed : {:.2?}",sort.1,now.elapsed());
    }
    /*let now: Instant = Instant::now();
    run_benchmark(1, 10, strand_sort);
    println!("{} Elapsed : {:.2?}","strand_sort",now.elapsed());*/
    
}