import Home from './component/Home.jsx';
import {renderToString} from 'react-dom/server';
import React from 'react';

const content = renderToString(React.createElement(Home));
console.log(content)