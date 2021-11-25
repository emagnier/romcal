const npmPublish = require('@jsdevtools/npm-publish');
const fs = require('fs');
const path = require('path');

(async () => {
  console.log(` - Publishing romcal`);
  const data = await npmPublish({
    package: path.join(__dirname, '..', 'package.json'),
    token: '${{ secrets.NPM_TOKEN }}',
    tag: 'dev',
    access: 'public',
    dryRun: true,
  });

  console.log(data);

  const bundlesBasePath = path.join(__dirname, '..', 'dist', 'bundles');
  const bundlePaths = fs
    .readdirSync(bundlesBasePath, { withFileTypes: true })
    .filter((dirent) => dirent.isDirectory())
    .map((dirent) => dirent.name);

  for (let i = 0; i < bundlePaths.length; i++) {
    console.log(` - Publishing bundle: ${bundlePaths[i]}`);
    const data = await npmPublish({
      package: path.join(bundlesBasePath, bundlePaths[i], 'package.json'),
      token: '${{ secrets.NPM_TOKEN }}',
      tag: 'dev',
      access: 'public',
      dryRun: true,
    });
    console.log(data);
  }
})();
