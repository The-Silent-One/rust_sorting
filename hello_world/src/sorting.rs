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
    let mut res = arr.to_vec();
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
