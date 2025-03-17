const { spawn } = require('child_process');
const path = require('path');

const rustBackendPath = path.resolve(__dirname, '../rust-backend');

// First, run database migrations
const migrateProcess = spawn('cargo', ['run', '--', 'migrate'], {
  cwd: rustBackendPath,
  shell: true,
  stdio: 'inherit',
});

migrateProcess.on('close', (code) => {
  if (code !== 0) {
    console.error(`Database migration failed with code ${code}`);
    process.exit(code);
  }
  
  console.log('Database migrations completed successfully.');
  
  // Now load fixtures
  const fixturesProcess = spawn('cargo', ['run', '--', 'load-fixtures'], {
    cwd: rustBackendPath,
    shell: true,
    stdio: 'inherit',
  });
  
  fixturesProcess.on('close', (code) => {
    if (code !== 0) {
      console.error(`Loading fixtures failed with code ${code}`);
      process.exit(code);
    }
    
    console.log('Fixtures loaded successfully.');
  });
}); 