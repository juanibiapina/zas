var http = require('http');

var port = process.argv[2];

function handleRequest(request, response){
  response.write("MOCK OTHER " + request.method + ": Url: " + request.url + "\n");

  if (request.method == "POST") {
    request.on("data", function(chunk) {
      response.write(chunk.toString());
    });

    request.on("end", function() {
      response.end();
    });
  } else {
    response.end();
  }
}

var server = http.createServer(handleRequest);

server.listen(port, function(){
  console.log("Server listening on: http://localhost:%s", port);
});
