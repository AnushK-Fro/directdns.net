import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { ProxySessions } from './pages/ProxySessions';

const App = () => {
  return (
    <Router>
      <Routes>
        {/* Single route for the entire UI */}
        <Route path="/" element={<ProxySessions />} />
      </Routes>
    </Router>
  );
};

export default App;
