import React from 'react';

class Page extends React.Component {

  render() {
    const { dataList = [] } = this.props;
    return (
      <div>
        <div>This is page</div>
      </div>
    )
  }
}

export default Page;