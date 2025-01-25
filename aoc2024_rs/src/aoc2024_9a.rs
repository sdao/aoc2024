#[derive(Clone, Copy, Debug)]
enum Block
{
    File(u32, i32),
    Free(u32),
}

pub fn main(inputs: Vec<String>)
{
    let mut blocks = Vec::<Block>::new();
    let mut num_files = 0;
    // let mut num_free = 0;
    let mut free = false;
    for c in inputs.first().unwrap().chars() {
        let size: u32 = c.to_digit(10).unwrap();
        if free {
            if size > 0
            {
                blocks.push(Block::Free(size));
            }
            // num_free += 1;
        }
        else {
            if size > 0
            {
                blocks.push(Block::File(size, num_files));
            }
            num_files += 1;
        }

        free = !free;
    }

    println!("{:?}", blocks);

    let mut cur_free = 0;
    loop
    {
        // clear all free blocks at end
        while let Some(Block::Free(_)) = blocks.last()
        {
            blocks.pop();
        }

        // find next free block
        while cur_free < blocks.len()
        {
            if let Block::Free(_) = blocks[cur_free]
            {
                break;
            }

            cur_free += 1;
        }

        if cur_free >= blocks.len()
        {
            break;
        }

        let file_block = blocks.pop().unwrap();
        let free_block = blocks[cur_free];

        if let Block::File(size, id) = file_block
        {
            if let Block::Free(freesize) = free_block
            {
                if freesize == size
                {
                    // replace free block with file block
                    blocks[cur_free] = file_block;
                }
                else if freesize > size
                {
                    // insert file block before free block
                    let free_remain = freesize - size;
                    blocks.insert(cur_free, file_block);
                    blocks[cur_free+1] = Block::Free(free_remain);
                }
                else
                {
                    // replace free block with partial file block, and re-push remaining file
                    let file_remain = size - freesize;
                    blocks[cur_free] = Block::File(freesize, id);
                    blocks.push(Block::File(file_remain, id));
                }
            }
            else {
                panic!()
            }
        }
        else {
            panic!()
        }

        // println!("{:?}", blocks);
    }

    let mut checksum = 0u64;
    let mut offset = 0usize;
    for block in blocks
    {
        match block
        {
            Block::Free(freesize) => {
                offset += freesize as usize;
            },
            Block::File(filesize, id) => {
                for i in offset..(offset+(filesize as usize)) {
                    checksum += ((i as u64) * (id as u64)) as u64;
                }
                offset += (filesize as usize);
            },
        }
    }

    println!("{}", checksum);
}