fn main() {
    // Line comments start with two lines
    // Everything after the slash is ignored by the compiler


    // Example: This line wont execute
    // println!("Hello, world! -- this line should not execute unless uncommented");
    
    /*

     Block comments temporarily disable code
     They can also be nested /* like this */ which makes it easy to comment out larfe sections
     quickly

     */


     /*
      * N.B. asterisks down the column are just for style and not required by the language
      *
      */

     // Block comments enable the toggling of code on/off by adding 
     // or removing just one slash: example below -> one slash added to uncomment code in a block

     //* // the one slash is added or removed here at the opening comment.... and other two at the
        // closing comment add the slash at the start to execute code and remove to keep block
        // comment

       println!("Now");
       println!("everything");
       println!("executes!");
      // Line comments inside remain unaffected
      
      // */

     // Block comments can also be used within expressions

     let x = 5 + /* 90 + */ 5;
     println!("Is `x` 10 or 100? x = {}", x);
}
