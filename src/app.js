// App.js
import React, { useEffect, useState } from 'react';
import axios from 'axios';

function App() {
  const [vendors, setVendors] = useState([]);

  useEffect(() => {
    axios.get('/api/vendors?location=NewYork&sort_by=reviews')
      .then(response => {
        setVendors(response.data);
      })
      .catch(error => {
        console.error("Error fetching vendors:", error);
      });
  }, []);

  return (
    <div>
      <h1>Vendors</h1>
      <ul>
        {vendors.map(vendor => (
          <li key={vendor.id}>
            {vendor.name} - {vendor.location}
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
