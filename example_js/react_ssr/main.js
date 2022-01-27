import React from 'react';
import {renderToString} from 'react-dom/server';

import Home from './component/Home.jsx';

const content = renderToString(React.createElement(Home));
console.log(content);
