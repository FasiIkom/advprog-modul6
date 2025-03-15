## Reflection on the `handle_connection` Method (1st commit)

The `handle_connection` method is responsible for processing an incoming TCP stream and handling the HTTP request sent by the client. Here's a breakdown of what happens inside this method:

1. **Buffered Reading of the Stream**:
   - A `BufReader` is created to wrap the incoming `TcpStream`. This allows for efficient reading of the stream line by line.

2. **Parsing the HTTP Request**:
   - The method reads lines from the stream using the `lines()` iterator.
   - Each line is unwrapped to handle potential errors during reading.
   - The `take_while` function is used to collect lines until an empty line is encountered, which signifies the end of the HTTP request headers.
   - The collected lines are stored in a `Vec<String>` called `http_request`.

3. **Debugging the Request**:
   - The parsed HTTP request is printed to the console in a formatted manner using `println!` and the `:#?` debug formatter. This is useful for debugging and understanding the structure of the incoming request.

### Key Points
- The method currently only reads and prints the HTTP request but does not process it further (e.g., routing or responding to specific requests).
- It assumes that the incoming data is valid UTF-8 and does not handle potential errors gracefully (e.g., malformed requests or connection issues).
- This implementation is a good starting point for building a basic HTTP server, but it can be extended to include features like request parsing, routing, and response generation.

This method demonstrates the foundational steps of handling HTTP requests in a Rust-based server.

## Reflection on the `handle_connection` Method (2nd commit)

In the second iteration of the `handle_connection` method, I have gained a deeper understanding of how HTTP responses are constructed and how the server communicates with the browser. Here are the key points I learned:

1. **Constructing an HTTP Response**:
   - The response consists of three main parts: the status line, headers, and the body.
   - The `Content-Length` header is critical as it informs the browser about the size of the response body. Without this header, the browser might not render the content correctly.

2. **Reading and Serving Static Files**:
   - The method uses `fs::read_to_string` to read the contents of `hello.html`. This demonstrates how to serve static files from the server.
   - The file's content is included in the response body, making it accessible to the client.

3. **Formatting the Response**:
   - The `format!` macro is used to dynamically construct the HTTP response string. This approach is simple and effective for small-scale servers.

4. **Error Handling**:
   - While the method works as intended, it uses `unwrap()` for error handling, which can cause the server to panic if an error occurs (e.g., if `hello.html` is missing). This highlights the need for robust error handling in production systems.

5. **Scalability and Extensibility**:
   - The current implementation is limited to serving a single static file (`hello.html`). To make the server more useful, it could be extended to handle multiple routes, dynamic content, or different HTTP methods.

### Key Takeaways
- This iteration of the `handle_connection` method has reinforced the importance of understanding HTTP headers and their role in client-server communication.
- It has also highlighted the need for better error handling and the potential for extending the server's functionality.
- Overall, this method serves as a foundational step toward building a more robust and feature-rich HTTP server in Rust.

### Screen capture of HTML returned
![Commit 2 screen capture](/assets/images/commit2.png)

## Reflection on Splitting Response and Refactoring (3rd commit)

During the development process, I realized that the original implementation of the `handle_connection` method was handling multiple responsibilities in one function. This includes parsing the HTTP request, determining the appropriate response, reading the static HTML file from disk, and constructing the final HTTP response. By splitting these responsibilities, the code becomes more modular and easier to maintain.

### Why Refactoring is Needed

- **Separation of Concerns:**  
  By separating the tasks (parsing the request, building the response, reading the file), each function now has a single responsibility. This adheres to the Single Responsibility Principle and promotes cleaner, more readable code.

- **Improved Error Handling:**  
  The original method used `unwrap()` frequently, which can lead to panics if an error occurs (for example, when a file is missing). Splitting the response building into its own function allows for a more straightforward implementation of error handling and recovery.

- **Scalability and Extensibility:**  
  With modular functions, it becomes easier to extend the server in the future. For instance, new routes, dynamic responses, or support for additional HTTP methods can be added without overloading a single function with too many tasks.

### How to Split the Response

1. **Parsing the Request:**  
   A dedicated function (`parse_request`) reads the incoming stream, extracts the HTTP request line, and returns it. This isolates the logic for processing user input.

2. **Building the Response:**  
   The `build_response` function is responsible for reading the appropriate HTML file based on the request and constructing a formatted HTTP response string. This function handles file IO and ensures that HTTP headers (like `Content-Length`) are set correctly.

3. **Handling Connection:**  
   Finally, the `handle_connection` function orchestrates the process by calling the `parse_request` and `build_response` functions, then writing the final response back to the client.

This refactoring not only improves code clarity but also lays the foundation for more robust server features in the future.

### Screen capture of bad request HTML returned
![Commit 3 screen capture](/assets/images/commit3.png)

## Reflection on Handling Concurrent Requests (4th commit)

When testing the server with two browser windows, one accessing `http://127.0.0.1:7878/sleep` and the other accessing `http://127.0.0.1:7878/`, it becomes evident that the server processes requests sequentially. Here's what happens:

1. **Sequential Processing**:
   - When the `/sleep` route is accessed, the server pauses for 10 seconds due to the `thread::sleep(Duration::from_secs(10))` call.
   - During this time, the server cannot handle other incoming requests, causing the second browser window (accessing `/`) to wait until the `/sleep` request is completed.

2. **Impact on User Experience**:
   - If multiple users access the server simultaneously, the sequential processing will cause delays for all users. This is because the server is currently single-threaded and processes one request at a time.

3. **Why It Works Like This**:
   - The server uses a single thread to handle all incoming requests. When a request involves a blocking operation (like `thread::sleep`), the thread is occupied and cannot process other requests until the operation is complete.

## Reflection on Multithreaded Server Using ThreadPool (5th commit)

After observing the limitations of a single-threaded server, we implemented a multithreaded server using a `ThreadPool`. This approach allows the server to handle multiple requests concurrently, significantly improving its responsiveness and scalability.

### Key Insights

1. **How ThreadPool Works**:
   - A `ThreadPool` initializes a fixed number of worker threads that wait for jobs.
   - When a new connection arrives, the server submits a job (the connection handling) to the pool. Each worker thread picks up a job from the queue and executes it.
   - This ensures that multiple requests can be processed simultaneously, without one blocking the others.

2. **Concurrency and Efficiency**:
   - By using a `ThreadPool`, the server can handle multiple requests concurrently, even if one request involves a blocking operation like `thread::sleep`.
   - Threads are reused for multiple jobs, avoiding the overhead of creating and destroying threads for each request.

3. **Improved User Experience**:
   - With a multithreaded server, users no longer experience delays caused by other requests. For example, accessing `/sleep` in one browser window no longer blocks access to `/` in another window.

4. **Resource Management**:
   - The `ThreadPool` efficiently manages system resources by limiting the number of threads to a fixed size. This prevents the server from being overwhelmed by too many simultaneous requests.