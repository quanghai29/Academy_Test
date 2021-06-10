// const restrict = require('./auth.mdw');
const swaggerUI = require('swagger-ui-express');
const {openapiSpecification} = require('../utils/swagger');

module.exports = function (app) {
  // link default
  app.get('/', (req, res) => {
    res.json({
      message: 'Hello from Online Course API',
    });
  });
 
  app.get('/view/course', (req, res) => {
    res.render('vwCourse/index.hbs', {
      title: 'Course'
    });
  });

  app.get('/view/course/add', (req, res) => {
    res.render('vwCourse/add.hbs', {
      title: 'Add Course'
    });
  });

  // link api documents
  app.use("/api-docs",
    swaggerUI.serve,
    swaggerUI.setup(openapiSpecification, { explorer: true }
  ));
  
  //routers path here
  app.use('/course', require('../routes/course.route'));
  app.use('/lecturer/course', require('../routes/lecturer/course.route'));
  app.use('/lecturer/chapter', require('../routes/lecturer/chapter.route'));
  app.use('/category',require('../routes/category.route'));
};
