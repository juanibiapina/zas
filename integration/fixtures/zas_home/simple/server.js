var http = require('http');

var port = process.argv[2];

function handleRequest(request, response){
  response.end("MOCK SIMPLE GET");
}

var server = http.createServer(handleRequest);

server.listen(port, function(){
  console.log("Server listening on: http://localhost:%s", port);
});
