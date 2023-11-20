// Serve the Dist folder
let express = require('express');

let path = require('path');

let app = express();

app.use('/', express.static(path.join(__dirname, 'dist')));

let port = process.env.PORT || 80;

app.listen(port);

console.log('Server started on port ' + port);