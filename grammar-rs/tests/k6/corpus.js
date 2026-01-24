// Corpus de textes pour tests K6
// Réutilise les bench_texts existants + variantes avec erreurs

// ============================================================================
// SHORT TEXTS (~100 mots)
// ============================================================================

const SHORT_CLEAN = `The quick brown fox jumps over the lazy dog. This sentence contains every letter of the alphabet and is commonly used for typing practice. Grammar checkers need to handle simple sentences like this one efficiently. The fox was very quick and the dog was quite lazy. They lived in a forest near a small village. Every day the fox would run through the meadow while the dog preferred to rest under the old oak tree. Sometimes they would meet by the river and watch the sunset together. Life in the countryside was peaceful and quiet.`;

const SHORT_ERRORS = `The quick brown fox jumps over the lazy dog. This sentance contains every letter of the alphabet and is commonly use for typing practice. Grammar checkers needs to handle simple sentences like this one efficiently. The fox was very quick and the dog was quite lazy. They lived in a forest near a small village. Every day the fox would runs through the meadow while the dog prefered to rest under the old oak tree. Sometimes they would meet by the river and watch the sunset together. Lifes in the countryside was peaceful and quiet.`;

const SHORT_FR = `Le renard brun rapide saute par-dessus le chien paresseux. Cette phrase contient chaque lettre de l'alphabet et est couramment utilisée pour la pratique de la frappe. Les vérificateurs de grammaire doivent gérer des phrases simples comme celle-ci de manière efficace. Le renard était très rapide et le chien était assez paresseux. Ils vivaient dans une forêt près d'un petit village. Chaque jour, le renard courait à travers la prairie pendant que le chien préférait se reposer sous le vieux chêne.`;

// ============================================================================
// MEDIUM TEXTS (~500 mots)
// ============================================================================

const MEDIUM_CLEAN = `The history of natural language processing has its roots in computational linguistics. Early attempts at machine translation in the 1950s led to significant research in how computers might understand and generate human language. The Georgetown experiment in 1954 demonstrated automatic translation of more than sixty Russian sentences into English. This early success led to significant funding and optimism in the field.

However, the ALPAC report in 1966 found that machine translation was nowhere near achieving its goals and recommended reducing funding for the research. This report led to a significant decrease in research activity, known as the first AI winter. Despite this setback, researchers continued to make progress in understanding language structure and developing parsing algorithms.

The 1970s and 1980s saw the development of conceptual ontologies and knowledge representation schemes. These approaches attempted to capture the meaning of language in structured formats that computers could reason about. Systems like SHRDLU demonstrated impressive capabilities in limited domains, but failed to scale to broader language understanding tasks.

The statistical revolution in the 1990s transformed the field. Rather than relying on hand-crafted rules, researchers began to use large corpora of text to train probabilistic models. Hidden Markov models became popular for part-of-speech tagging, while statistical machine translation systems achieved better results than their rule-based predecessors.

The introduction of word embeddings in the 2010s represented another paradigm shift. Models like Word2Vec and GloVe learned dense vector representations of words from large text corpora. These representations captured semantic relationships between words, enabling new approaches to many NLP tasks.

Deep learning has dominated the field since the mid-2010s. Recurrent neural networks, particularly LSTM and GRU architectures, became the standard approach for sequential language modeling. The attention mechanism, introduced in 2014, allowed models to focus on relevant parts of the input when generating outputs.

The transformer architecture, introduced in 2017, revolutionized natural language processing. By relying entirely on attention mechanisms without recurrence, transformers enabled much more efficient training on large datasets. BERT, GPT, and their successors have achieved state-of-the-art results on virtually every NLP benchmark.

Today, large language models trained on billions of parameters demonstrate remarkable capabilities in text generation, translation, summarization, and question answering. These models have moved from research laboratories into practical applications, powering search engines, virtual assistants, and writing tools used by millions of people daily.`;

const MEDIUM_ERRORS = `The history of natural language processing has it's roots in computational linguistics. Early attempts at machine translation in the 1950s leaded to significant research in how computers might understand and generate human language. The Georgetown experiment in 1954 demonstrated automatic translation of more then sixty Russian sentences into English. This early success led to significant funding and optimism in the field.

However, the ALPAC report in 1966 found that machine translation was nowhere near achieving it's goals and recommended reducing funding for the research. This report led to a significant decrease in research activity, known as the first AI winter. Despite this setback, researches continued to make progress in understanding language structure and developing parsing algorithms.

The 1970s and 1980s saw the development of conceptual ontologies and knowledge representation schemes. These approaches attempted to capture the meaning of language in structured formats that computers could reason about. Systems like SHRDLU demonstrated impressive capabilities in limited domains, but failed to scale to broader language understanding tasks.

The statistical revolution in the 1990s transformed the field. Rather then relying on hand-crafted rules, researchers began to use large corpora of text to train probabilistic models. Hidden Markov models became popular for part-of-speech tagging, while statistical machine translation systems achieved better results then their rule-based predecessors.

The introduction of word embeddings in the 2010s represented another paradigm shift. Models like Word2Vec and GloVe learned dense vector representations of words from large text corpora. These representations captured semantic relationships between words, enabling new approaches to many NLP tasks.

Deep learning has dominated the field since the mid-2010s. Recurrent neural networks, particularly LSTM and GRU architectures, became the standard approach for sequential language modeling. The attention mechanism, introduced in 2014, allowed models to focus on relevant parts of the input when generating outputs.

The transformer architecture, introduced in 2017, revolutionized natural language processing. By relying entirely on attention mechanisms without recurrence, transformers enabled much more efficient training on large datasets. BERT, GPT, and their successors have achieved state-of-the-art results on virtually every NLP benchmark.

Today, large language models trained on billions of parameters demonstrate remarkable capabilities in text generation, translation, summarization, and question answering. These models have moved from research laboratorys into practical applications, powering search engines, virtual assistants, and writing tools used by millions of people daily.`;

// ============================================================================
// LONG TEXTS (~1500 mots)
// ============================================================================

const LONG_CLEAN = `The Evolution of Computing Technology

The story of computing technology spans centuries of human innovation and ingenuity. From the earliest mechanical calculators to modern quantum computers, each generation has built upon the discoveries of its predecessors to create increasingly powerful tools for information processing.

The prehistory of computing begins with simple counting devices. The abacus, developed independently in ancient civilizations across the world, represents one of the earliest tools for mathematical calculation. These simple devices enabled merchants and scholars to perform arithmetic operations with greater speed and accuracy than mental calculation alone.

The seventeenth century saw the development of mechanical calculators by mathematicians and inventors. Blaise Pascal created the Pascaline in 1642, a machine capable of addition and subtraction. Gottfried Wilhelm Leibniz improved upon this design, creating a machine that could perform all four basic arithmetic operations. These mechanical calculators laid the groundwork for more sophisticated computing devices.

Charles Babbage, an English mathematician, designed the first general-purpose computing machines in the nineteenth century. His Difference Engine, conceived in 1822, was designed to calculate polynomial functions automatically. The Analytical Engine, designed later, incorporated many features of modern computers, including a central processing unit, memory, and the ability to be programmed using punched cards. Although neither machine was completed during his lifetime, Babbage's designs anticipated many developments that would come a century later.

Ada Lovelace, who collaborated with Babbage, is often credited as the first computer programmer. Her notes on the Analytical Engine included an algorithm for calculating Bernoulli numbers, making her the first person to recognize that a computing machine could be used for more than just calculation. Her vision of computing as a general-purpose tool for manipulating symbols would prove prophetic.

The late nineteenth and early twentieth centuries brought advances in electrical engineering that would transform computing. Herman Hollerith developed electromechanical tabulating machines for the 1890 United States Census, dramatically reducing the time required to process census data. His company would eventually become IBM, one of the most important computer companies in history.

The theoretical foundations of modern computing were established in the 1930s. Alan Turing's concept of a universal computing machine provided a mathematical model for computation itself. Turing demonstrated that a simple machine following a finite set of rules could, in principle, compute anything that could be computed. This insight remains fundamental to computer science today.

The Second World War accelerated the development of electronic computing machines. The urgent need to break enemy codes and calculate artillery firing tables drove governments to invest heavily in computing research. The Colossus computers, built in Britain to decrypt German messages, were among the first electronic digital computers. In the United States, the ENIAC, completed in 1945, was designed to calculate artillery firing tables and later used for other scientific calculations.

The post-war period saw rapid advances in computer hardware and software. The stored-program concept, which allows computers to store both programs and data in the same memory, enabled much more flexible and powerful machines. John von Neumann's architecture, which separates the central processing unit from memory, became the standard model for computer design.

The invention of the transistor at Bell Labs in 1947 marked the beginning of solid-state electronics. Transistors were smaller, faster, more reliable, and used less power than the vacuum tubes they replaced. This technology enabled the development of smaller and more powerful computers throughout the 1950s and 1960s.

The integrated circuit, invented independently by Jack Kilby and Robert Noyce in the late 1950s, represented another quantum leap in computing technology. By combining multiple transistors and other components on a single chip of semiconductor material, integrated circuits dramatically reduced the cost and size of electronic systems while increasing their reliability and performance.

The 1960s and 1970s saw the emergence of operating systems, programming languages, and networking technologies that would define modern computing. Time-sharing systems allowed multiple users to share a single computer simultaneously. Languages like COBOL, FORTRAN, and later C provided abstractions that made programming more accessible. The ARPANET, predecessor to the Internet, demonstrated the feasibility of wide-area computer networking.

The personal computer revolution of the late 1970s and 1980s brought computing power to homes and small businesses. The Apple II, Commodore 64, IBM PC, and their successors made computers accessible to ordinary people. Graphical user interfaces, pioneered at Xerox PARC and popularized by Apple and Microsoft, made computers easier to use for non-technical users.

The World Wide Web, invented by Tim Berners-Lee in 1989, transformed the Internet from a tool for researchers into a global information infrastructure. The combination of hypertext, universal addressing, and a simple protocol for transferring documents enabled explosive growth in online information sharing and communication. Web browsers and search engines made this information accessible to anyone with an Internet connection.`;

const LONG_ERRORS = `The Evolution of Computing Technology

The story of computing technology spans centurys of human innovation and ingenuity. From the earliest mechanical calculators to modern quantum computers, each generation has builded upon the discoveries of it's predecessors to create increasingly powerful tools for information processing.

The prehistory of computing begins with simple counting devices. The abacus, developed independently in ancient civilizations across the world, represents one of the earliest tools for mathematical calculation. These simple devices enabled merchants and scholars to perform arithmetic operations with greater speed and accuracy then mental calculation alone.

The seventeenth century saw the development of mechanical calculators by mathematicians and inventors. Blaise Pascal created the Pascaline in 1642, a machine capable of addition and subtraction. Gottfried Wilhelm Leibniz improved upon this design, creating a machine that could perform all four basic arithmetic operations. These mechanical calculators layed the groundwork for more sophisticated computing devices.

Charles Babbage, an English mathematician, designed the first general-purpose computing machines in the nineteenth century. His Difference Engine, conceived in 1822, was designed to calculate polynomial functions automatically. The Analytical Engine, designed later, incorporated many features of modern computers, including a central processing unit, memory, and the ability to be programmed using punched cards. Although neither machine was completed during his lifetime, Babbage's designs anticipated many developments that would come a century later.

Ada Lovelace, who collaborated with Babbage, is often credited as the first computer programmer. Her notes on the Analytical Engine included an algorithm for calculating Bernoulli numbers, making her the first person to recognize that a computing machine could be used for more then just calculation. Her vision of computing as a general-purpose tool for manipulating symbols would prove prophetic.

The late nineteenth and early twentieth centuries brought advances in electrical engineering that would transform computing. Herman Hollerith developed electromechanical tabulating machines for the 1890 United States Census, dramatically reducing the time required to process census data. His company would eventually become IBM, one of the most important computer companys in history.

The theoretical foundations of modern computing was established in the 1930s. Alan Turing's concept of a universal computing machine provided a mathematical model for computation itself. Turing demonstrated that a simple machine following a finite set of rules could, in principle, compute anything that could be computed. This insight remains fundamental to computer science today.

The Second World War accelerated the development of electronic computing machines. The urgent need to break enemy codes and calculate artillery firing tables drived governments to invest heavily in computing research. The Colossus computers, built in Britain to decrypt German messages, was among the first electronic digital computers. In the United States, the ENIAC, completed in 1945, was designed to calculate artillery firing tables and later used for other scientific calculations.

The post-war period saw rapid advances in computer hardware and software. The stored-program concept, which allows computers to store both programs and data in the same memory, enabled much more flexible and powerful machines. John von Neumann's architecture, which separates the central processing unit from memory, became the standard model for computer design.

The invention of the transistor at Bell Labs in 1947 marked the beginning of solid-state electronics. Transistors was smaller, faster, more reliable, and used less power then the vacuum tubes they replaced. This technology enabled the development of smaller and more powerful computers throughout the 1950s and 1960s.

The integrated circuit, invented independently by Jack Kilby and Robert Noyce in the late 1950s, represented another quantum leap in computing technology. By combining multiple transistors and other components on a single chip of semiconductor material, integrated circuits dramatically reduced the cost and size of electronic systems while increasing their reliability and performance.

The 1960s and 1970s saw the emergence of operating systems, programming languages, and networking technologies that would define modern computing. Time-sharing systems allowed multiple users to share a single computer simultaneously. Languages like COBOL, FORTRAN, and later C provided abstractions that made programming more accessible. The ARPANET, predecessor to the Internet, demonstrated the feasibility of wide-area computer networking.

The personal computer revolution of the late 1970s and 1980s brought computing power to homes and small businesses. The Apple II, Commodore 64, IBM PC, and their successors made computers accessible to ordinary people. Graphical user interfaces, pioneered at Xerox PARC and popularized by Apple and Microsoft, made computers easier to use for non-technical users.

The World Wide Web, invented by Tim Berners-Lee in 1989, transformed the Internet from a tool for researchers into a global information infrastructure. The combination of hypertext, universal addressing, and a simple protocol for transferring documents enabled explosive growth in online information sharing and communication. Web browsers and search engines made this information accessible to anyone with an Internet connection.`;

// ============================================================================
// CORPUS STRUCTURE
// ============================================================================

export const CORPUS = {
  short: [
    { text: SHORT_CLEAN, lang: 'en', expectedErrors: 0 },
    { text: SHORT_ERRORS, lang: 'en', expectedErrors: 8 },
    { text: SHORT_FR, lang: 'fr', expectedErrors: 0 },
  ],
  medium: [
    { text: MEDIUM_CLEAN, lang: 'en', expectedErrors: 0 },
    { text: MEDIUM_ERRORS, lang: 'en', expectedErrors: 12 },
  ],
  long: [
    { text: LONG_CLEAN, lang: 'en', expectedErrors: 0 },
    { text: LONG_ERRORS, lang: 'en', expectedErrors: 18 },
  ],
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

export function getRandomText(tier = 'medium') {
  const texts = CORPUS[tier];
  if (!texts || texts.length === 0) {
    throw new Error(`Unknown tier: ${tier}`);
  }
  return texts[Math.floor(Math.random() * texts.length)];
}

export function getAllTexts() {
  return [
    ...CORPUS.short,
    ...CORPUS.medium,
    ...CORPUS.long,
  ];
}

export function getTextByTier(tier) {
  return CORPUS[tier] || [];
}
