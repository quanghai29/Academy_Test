const express = require('express');
const morgan = require('morgan');
const cors = require('cors');
const exphbs = require('express-handlebars');
const express_handlebars_sections = require('express-handlebars-sections');
require('express-async-errors');
require('dotenv').config();
const path = require('path');
const app = express();

app.use(cors());
app.use(express.json());

app.use(morgan('dev'));

app.use(express.static(path.join(__dirname, '/Contents')));
app.engine('hbs', exphbs(
    {
        defaultLayout: 'main.hbs',
        layoutsDir: 'views/_layouts',
        helpers:{
          section: express_handlebars_sections()
        }
    })
);
app.set('view engine', 'hbs');

// middleware api
require('./middlewares/routes.mdw')(app);
// middleware error
require('./middlewares/error.mdw')(app);

//require('./chatbot');

const { PORT } = process.env;
app.listen(PORT, () => {
  console.log(`Online Academy backend is runing at ${process.env.HOST_NAME}:${PORT}`);
});
