// Point to resource in memory - Pointer variables
pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3, 4];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    //With Non-Primitives, if you assign another variable to a piece of data, the first varible will no longer hold that value. You'll need to use a reference & point
    //to that resource

    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1; // Its a primitive data type. so directed assigning is not possible.
                      // adding the & will help you in making the primitive type to referene type
    println!("Values: {:?}", (&vec1, vec2));
}
