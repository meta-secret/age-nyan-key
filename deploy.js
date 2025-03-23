import { publish } from 'gh-pages';

publish('dist', {
  branch: 'gh-pages',
  message: 'Auto-deploy from script',
  dotfiles: true,
}, (err) => {
  if (err) {
    console.error('Deployment error:', err);
    return;
  }
  console.log('Successfully deployed to GitHub Pages!');
}); 