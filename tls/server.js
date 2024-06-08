const express = require('express');
const https = require('https');
const fs = require('fs');

const app = express();

// Загрузка сертификатов
const privateKey = fs.readFileSync(__dirname+'/cert/domain.key', 'utf8');
const certificate = fs.readFileSync(__dirname+'/cert/domain.crt', 'utf8');
const ca = fs.readFileSync(__dirname+'/cert/ruc.pem', 'utf8');

const credentials = {
 key: privateKey,
 cert: certificate,
 ca: ca
};

// Создание HTTPS сервера
const httpsServer = https.createServer(credentials, app);

// Определение маршрутов
app.get('/', (req, res) => {
 res.send('Hello, HTTPS!');
});

// Запуск сервера
httpsServer.listen(3000, () => {
 console.log('HTTPS Server running on port 3000');
});
