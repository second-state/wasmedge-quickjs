import React from 'react';
import Page from './Page.jsx';
 
class Home extends React.Component {

  render() {
    const { dataList = [] } = this.props;
    return (
      <div>
        <div>This is home</div>
        <Page></Page>
      </div>
    )
  }
}

export default Home;
