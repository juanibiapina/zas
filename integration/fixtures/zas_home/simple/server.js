var http = require("http");
var fs = require("fs");

var port = process.argv[2];

function handleRequest(request, response){
  if (request.url == "/save_headers") {
    var headers = "";
    var keys = Object.keys(request.headers);
    keys.sort();
    for (var i=0; i < keys.length; i++) {
      var key = keys[i];
      headers += key + ": " + request.headers[key] + "\n";
    }
    fs.writeFile("../../../../tmp/headers", headers);
    response.end();
    return;
  }

  response.write("MOCK SIMPLE " + request.method + ": Url: " + request.url + "\n");

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
