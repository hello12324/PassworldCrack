[中文文档](./README_CN.md)
# Explanation
  > This is my practice project for learning the rust language. Although the writing is not very good, I will work hard!.
  > If you think there is something wrong with my code please
  	 My email
		testsendkfotesycike@gmail.com
	Go to my mailbox to communicate!

  > PassworldCarck Actually, I want to implement this function on the arduino programming board (using the HID vulnerability (Human Interface Device)), because the programming board cannot be filled with too many bytes, so I want to realize the project of automatically generating passwords in real time , Now it has been implemented, using rust language, later I will use the implementation on arduino, and open the code to this repository.

# My project
  ## ANN(Artificial Neural Network) ----status (not executable, in progress)
  	Here is my simple artificial neural network, which is just a prototype and cannot be implemented
  
  ## leibniz_transformation ---- status (executable, to be optimized)
  	Leibniz transformation is a mathematical method for converting a time function into its complex plane function representation. Leibniz transformation is most commonly used to solve ordinary differential equations (differential equations), and is also commonly used to analyze electrical and electronic Signals in systems and system dynamic behavior.
	Specifically, the Leibniz transform can convert the differential and integral terms in the time function into the product and sum of persimmons (complex functions), so that the linear algebra method can be used to solve the problem. Leibniz transform It can also be used to analyze the steady-state behavior of the system (steady-state behavior), because in the steady state, the state of the system does not change anymore, and the complex spectrum analysis can be used to study the nature of the system. Not necessarily Correct, but that's about it.
  
  ## PassworldCrack ---- status (runnable, to be optimized)
  	This is a project that does not require a dictionary to generate passwords in real time. It only needs a "DefaultPassworld.txt" file (the file needs to be in the same directory as the program or in the path of the terminal, but the path can be customized at runtime), and there are 94 inside the file Characters (commonly used characters can also be customized) to generate a password, using the basic application of discrete mathematics, combinatorics (Combinatorics), as shown below

	## If there are 3 characters in total, to crack a 2-digit password, the arrangement and combination are as follows
		1. aa --
		2. ab   |
		3. ac   |
		4. b    |
		5. bb   | =>[there are 3 to the power of 2 in total]
		6. bc   |
		7. ca   |
		8. cb   |
		9. cc __|
	The use of linear arrangement is to consider the order, so there is still a problem with efficiency.
  
  ## PassworldProgram ----status (can run, to be optimized)
  	This is a program with a password, just simply write tests, such as time attack tests and so on.
  
  ## C Language Project ---- status (in progress)
          Passworld Crack Rewritten version
                 This is a rewritten version of the Rust language PassworldCrack project, because I found that the efficiency of the underlying language is much better than the rust language (maybe I have a problem with optimization), in short, I plan to rewrite this project in C language, and the functionality may be the same as the original almost.
          Passworld Crack Arduino Leomardo Version
                 This is the Arduino version of the Rust language PassworldCrack project, which allows brute force cracking to be used in daily life, using the HID (Human Interface Divce) vulnerability!

> If you want to see the details of the projects here, please see the release!.
> The items here have no use value, just for fun.
> One more word! Merry Cristmas!
> "The year is coming to an end, and we will all eventually grow old, but our determination and resolve will always exist! Keep going!"
